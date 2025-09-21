use crate::entity::Entity;
use crate::world::World;
use std::cell::{Ref, RefMut};
use std::marker::PhantomData;

trait QueryData {
    type Data<'w>;

    fn matches(world: &World, entity: Entity) -> bool;
    fn fetch<'w>(world: &'w World, entity: Entity) -> Option<Self::Data<'w>>;
}

impl<T: 'static> QueryData for &T {
    type Data<'w> = Ref<'w, T>;

    fn matches(world: &World, entity: Entity) -> bool {
        world.has_component::<T>(entity)
    }

    fn fetch<'w>(world: &'w World, entity: Entity) -> Option<Self::Data<'w>> {
        world.get_component::<T>(entity)
    }
}

impl<T: 'static> QueryData for &mut T {
    type Data<'w> = RefMut<'w, T>;

    fn matches(world: &World, entity: Entity) -> bool {
        world.has_component::<T>(entity)
    }

    fn fetch<'w>(world: &'w World, entity: Entity) -> Option<Self::Data<'w>> {
        world.get_component_mut::<T>(entity)
    }
}
macro_rules! query_data_impls {
    ($($T:ident),+) => {
        impl<$($T: QueryData),+> QueryData for ($($T),+) {
            type Data<'w> = ($($T::Data<'w>),+);

            fn matches(world: &World, entity: Entity) -> bool {
                $($T::matches(world, entity))&&+
            }

            fn fetch<'w>(world: &'w World, entity: Entity) -> Option<Self::Data<'w>> {
                Some((
                    $($T::fetch(world, entity)?),+
                ))
            }
        }
    };
}

query_data_impls!(A, B);
query_data_impls!(A, B, C);
query_data_impls!(A, B, C, D);
query_data_impls!(A, B, C, D, E);
query_data_impls!(A, B, C, D, E, F);
query_data_impls!(A, B, C, D, E, F, G);
query_data_impls!(A, B, C, D, E, F, G, H);

pub trait System {
    fn run(&mut self, world: &World);
}

pub struct SystemsManager {
    systems: Vec<Box<dyn System>>,
}

impl SystemsManager {
    pub fn new() -> Self {
        Self { systems: Vec::new() }
    }

    pub fn add_system<S: System + 'static>(&mut self, system: S) {
        self.systems.push(Box::new(system));
    }

    pub fn run(&mut self, world: &World) {
        for system in &mut self.systems {
            system.run(world);
        }
    }
}

struct Query<T: QueryData> {
    _q: PhantomData<T>
}

impl<'w, T: QueryData<Data<'w> = T>> Query<T> {
    pub fn iter(&self, world: &'w World) -> impl Iterator<Item=T> {
        (0..world.next_id).map(
            |e| T::fetch(world, Entity::new(e))
        ).flatten()
    }
}

trait SystemParam {
    fn fetch() -> Self;
}

impl<T: QueryData> SystemParam for Query<T> {
    fn fetch() -> Self {
        Self { _q: PhantomData::default() }
    }
}

pub struct FunctionSystem<F, Params> {
    func: F,
    _params: PhantomData<Params>
}

impl<F, Params> FunctionSystem<F, Params> {
    pub fn new(func: F) -> Self {
        Self {
            func,
            _params: PhantomData
        }
    }
}

macro_rules! impl_system {
    ($($param:ident),*) => {
        impl<F, $($param),*> System for FunctionSystem<F, ($($param,) *)>
        where
            F: FnMut($($param),*) + 'static,
            $($param: SystemParam),*
        {
            fn run(&mut self, _world: &World) {
                // self($( <$param as QueryData>::fetch_query(world) ),*);
                // self($( $param::tt() ),*);
                (self.func)($($param::fetch()), *)
            }
        }
    };
}

// Generate implementations for 0 to N query parameters
impl_system!();
impl_system!(A);
impl_system!(A, B);
impl_system!(A, B, C);
impl_system!(A, B, C, D);
impl_system!(A, B, C, D, E);

pub fn system<F, Params>(func: F) -> FunctionSystem<F, Params> {
    FunctionSystem::new(func)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::collections::sorted_vector::SortedVec;
    use std::any::TypeId;

    struct Comp1 {
        x: i32,
    }

    struct Comp2 {
        y: i32,
    }

    #[test]
    fn test_query() {
        let mut world = World::new();

        let entity1 = world.spawn();

        world.register_component::<Comp1>();
        world.register_component::<Comp2>();

        world.add_component(entity1, Comp1 { x: 1 });
        world.add_component(entity1, Comp2 { y: 2 });

        let entity2 = world.spawn();
        world.add_component(entity2, Comp2 { y: 2 });

        let entity3 = world.spawn();
        world.add_component(entity3, Comp1 { x: 1 });

        {
            let mut a: RefMut<Comp1> = <&mut Comp1 as QueryData>::fetch(&world, entity1).unwrap();
            a.x += 20;
        }
        let (b, d): (Ref<Comp1>, Ref<Comp2>) =
            <(&Comp1, &Comp2) as QueryData>::fetch(&world, entity1).unwrap();
        assert_eq!(b.x, 21);
        assert_eq!(d.y, 2);
        println!("{}, {}", b.x, d.y);
        let c = 20;

        assert!(<&Comp1 as QueryData>::fetch(&world, entity2).is_none());

        {
            let b: Vec<Entity> = world
                .get_components(
                    &[TypeId::of::<Comp1>()]
                        .into_iter()
                        .collect::<SortedVec<TypeId>>(),
                )
                .iter()
                .map(|(a, b)| *a)
                .collect::<Vec<Entity>>();
            let d = 20;
        }

    }

    struct MyComponent;
    // impl QueryData for MyComponent {}

    #[test]
    fn test_systems() {
        let mut manager = SystemsManager::new();
        let mut world = World::new();

        let entity1 = world.spawn();

        world.register_component::<Comp1>();
        world.register_component::<Comp2>();

        world.add_component(entity1, Comp1 { x: 1 });
        world.add_component(entity1, Comp2 { y: 2 });

        let entity2 = world.spawn();
        world.add_component(entity2, Comp2 { y: 3 });

        let entity3 = world.spawn();
        world.add_component(entity3, Comp1 { x: 4 });

        // Add systems using the wrapper
        manager.add_system(system(|| println!("System with no params")));
        // manager.add_system(system(|_q: Query<&MyComponent>, _q2: Query<&MyComponent>| println!("System with one param")));
        manager.add_system(system(|q: Query<&Comp1>| {
            println!("Running system 2")
        }));

        manager.run(&world);
    }
}

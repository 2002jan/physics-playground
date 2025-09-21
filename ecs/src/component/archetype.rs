use crate::component::registry::ComponentRegistry;
use crate::component::storage::{ComponentStorage, ComponentStorageAny};
use crate::entity::Entity;
use common::collections::sorted_vector::SortedVec;
use std::any::{Any, TypeId};
use std::cell::{Ref, RefMut};
use std::collections::HashMap;

pub struct Archetype {
    // signature: u64, // Uses hash of SortedVec<TypeId> of components
    types: SortedVec<TypeId>,
    entities: Vec<Entity>,
    storages: HashMap<TypeId, Box<dyn ComponentStorageAny>>,
}

impl Archetype {
    pub fn new(sig: &SortedVec<TypeId>, registry: &ComponentRegistry) -> Self {
        let mut storages = HashMap::new();

        for t in &*sig {
            storages.insert(
                t.to_owned(),
                registry
                    .create_component_storage(t)
                    .expect("Cannot create an archetype of unregistered component type"),
            );
        }

        Self {
            types: sig.clone(),
            entities: Vec::new(),
            storages,
        }
    }

    pub fn get_component<T: 'static>(&self, entity: Entity) -> Option<Ref<'_, T>> {
        let type_id = TypeId::of::<T>();

        let storage = self
            .storages
            .get(&type_id)?
            .as_any()
            .downcast_ref::<ComponentStorage<T>>()?;

        storage.get(entity)
    }

    pub fn get_component_mut<T: 'static>(&self, entity: Entity) -> Option<RefMut<'_, T>> {
        let type_id = TypeId::of::<T>();

        let storage = self
            .storages
            .get(&type_id)?
            .as_any()
            .downcast_ref::<ComponentStorage<T>>()?;

        storage.get_mut(entity)
    }

    pub fn add_entity(&mut self, entity: Entity, components: Vec<Box<dyn Any>>) {
        if components.len() != self.types.len() {
            panic!("Invalid components list")
        }

        self.entities.push(entity);

        for (i, c) in components.into_iter().enumerate() {
            let type_id = self.types[i];

            let storage = self
                .storages
                .get_mut(&type_id)
                .expect("Invalid component type for this archetype");

            storage.insert_any(entity, c);
        }
    }

    pub fn has_entity(&self, entity: Entity) -> bool {
        self.entities.contains(&entity)
    }

    pub fn has_component<T: 'static>(&self) -> bool {
        self.types.contains(&TypeId::of::<T>())
    }

    pub fn remove_entity(&mut self, entity: Entity) -> Option<Vec<Box<dyn Any>>> {
        let entity = self.entities.iter().position(|&e| e.eq(&entity))?;
        let entity = self.entities.swap_remove(entity);

        let mut components: Vec<Box<dyn Any>> = Vec::with_capacity(self.types.len() + 1);

        for (_, s) in &mut self.storages {
            let component = s.remove(entity).unwrap();
            components.push(component);
        }

        Some(components)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestComponent;

    fn get_registry() -> ComponentRegistry {
        let mut registry = ComponentRegistry::new();

        registry.register_component::<i32>();
        registry.register_component::<f32>();
        registry.register_component::<TestComponent>();

        registry
    }

    fn get_sig_vector() -> SortedVec<TypeId> {
        [
            TypeId::of::<i32>(),
            TypeId::of::<f32>(),
            TypeId::of::<TestComponent>(),
        ]
        .into_iter()
        .collect()
    }

    #[test]
    fn test_new() {
        let archetype = Archetype::new(
            &mut get_sig_vector(),
            &get_registry()
        );

        assert_eq!(archetype.types.as_slice(), get_sig_vector().as_slice());
        assert!(archetype.entities.is_empty());
        assert!(archetype.storages.contains_key(&TypeId::of::<i32>()));
        assert!(archetype.storages.contains_key(&TypeId::of::<f32>()));
        assert!(archetype.storages.contains_key(&TypeId::of::<TestComponent>()));
        assert!(!archetype.storages.contains_key(&TypeId::of::<usize>()));

        archetype.storages[&TypeId::of::<i32>()].as_any().downcast_ref::<ComponentStorage<i32>>().unwrap();
        archetype.storages[&TypeId::of::<f32>()].as_any().downcast_ref::<ComponentStorage<f32>>().unwrap();
        archetype.storages[&TypeId::of::<TestComponent>()].as_any().downcast_ref::<ComponentStorage<TestComponent>>().unwrap();
    }
}

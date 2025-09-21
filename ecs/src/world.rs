use crate::component::archetype::Archetype;
use crate::component::registry::ComponentRegistry;
use crate::entity::Entity;
use common::collections::sorted_vector::SortedVec;
use std::any::{Any, TypeId};
use std::cell::{Ref, RefMut};
use std::collections::HashMap;

pub struct World {
    pub next_id: u32,
    component_registry: ComponentRegistry,
    archetypes: HashMap<SortedVec<TypeId>, Archetype>,
    entities: HashMap<Entity, SortedVec<TypeId>>,
}

impl World {
    pub fn new() -> Self {
        let component_registry = ComponentRegistry::new();
        let mut archetypes = HashMap::new();
        let empty_type = SortedVec::new();
        let archetype = Archetype::new(&empty_type, &component_registry);

        archetypes.insert(
            empty_type,
            archetype
        );

        Self {
            next_id: 0,
            component_registry,
            archetypes,
            entities: HashMap::new(),
        }
    }

    pub fn spawn(&mut self) -> Entity {
        let id = self.next_id;
        self.next_id += 1;

        let entity = Entity::new(id);
        let type_ids = SortedVec::new();
        self.archetypes.get_mut(&type_ids).unwrap().add_entity(entity, Vec::new());
        self.entities.insert(
            entity,
            SortedVec::new(),
        );

        entity
    }

    pub fn register_component<T: 'static>(&mut self) {
        self.component_registry.register_component::<T>();
    }

    pub fn add_component<T: 'static>(&mut self, entity: Entity, component: T) {
        let mut old_types = self
            .entities
            .remove(&entity)
            .expect("(Add proper handling) entity is dead");

        let mut components = self
            .archetypes
            .get_mut(&old_types)
            .expect("Archetype should have been registered")
            .remove_entity(entity)
            .expect("Entity should have registered in an archetype");

        old_types.insert(TypeId::of::<T>());
        components.push(Box::new(component));

        components.sort_by_cached_key(|c| (*c).type_id());

        self.entities.insert(entity, old_types.clone());
        let archetype = self
            .archetypes
            .entry(old_types.clone())
            .or_insert_with(|| Archetype::new(&mut old_types, &mut self.component_registry));

        archetype.add_entity(entity, components);
    }

    pub fn has_component_id(&self, entity: Entity, type_id: &TypeId) -> bool {
        match self.entities.get(&entity) {
            None => false,
            Some(types) => types.contains(type_id),
        }
    }

    pub fn has_component<T: 'static>(&self, entity: Entity) -> bool {
        self.has_component_id(entity, &TypeId::of::<T>())
    }

    pub fn get_component<T: 'static>(&self, entity: Entity) -> Option<Ref<'_, T>> {
        let types = self.entities.get(&entity)?;

        self.archetypes.get(types)?.get_component(entity)
    }

    pub fn get_component_mut<T: 'static>(&self, entity: Entity) -> Option<RefMut<'_, T>> {
        let types = self.entities.get(&entity)?;

        self.archetypes.get(types)?.get_component_mut(entity)
    }

    pub fn get_components(&self, types: &SortedVec<TypeId>) -> Vec<(Entity, Vec<Box<dyn Any>>)> {
        let output = Vec::new();

        for (t, _a) in &self.archetypes {
            if !types.is_subset(&t) {
                continue;
            }
        }

        output
    }
}

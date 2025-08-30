use std::any::Any;
use std::collections::HashMap;
use crate::component::Component;
use crate::entity::Entity;

pub trait ComponentStorageAny: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn remove(&mut self, entity: Entity) -> bool;
    fn insert_any(&mut self, entity: Entity, component: Box<dyn Any>);
}

pub struct ComponentStorage<T: Component> {
    storage: HashMap<Entity, T>,
}

impl<T: Component> ComponentStorageAny for ComponentStorage<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn remove(&mut self, entity: Entity) -> bool {
        self.storage.remove(&entity).is_some()
    }

    fn insert_any(&mut self, entity: Entity, component: Box<dyn Any>) {
        self.storage.insert(
            entity,
            *component.downcast::<T>().expect("Component missmatch")
        );
    }
}

impl<T: Component> ComponentStorage<T> {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    fn insert(&mut self, entity: Entity, component: T) {
        self.storage.insert(
            entity,
            component,
        );
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        self.storage.get(&entity)
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        self.storage.get_mut(&entity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestComponent {}
    struct OtherComponent {}

    #[test]
    fn test_new() {
        let storage = ComponentStorage::<TestComponent>::new();

        assert!(storage.storage.is_empty());
    }

    #[test]
    fn test_insert() {
        let mut storage = ComponentStorage::<i32>::new();
        let test_entity = Entity::new(1);

        storage.insert(test_entity, 21);
        assert!(storage.storage.contains_key(&test_entity));
        assert_eq!(storage.get(test_entity), Some(&21));
    }

    #[test]
    fn test_get() {
        let mut storage = ComponentStorage::<i32>::new();
        let test_entity_1 = Entity::new(1);
        let test_entity_2 = Entity::new(2);
        let test_entity_3 = Entity::new(3);
        let test_entity_nope = Entity::new(4);

        storage.insert(test_entity_1, 30);
        storage.insert(test_entity_2, 8);
        storage.insert(test_entity_3, 2025);
        assert_eq!(storage.get(test_entity_1), Some(&30));
        assert_eq!(storage.get(test_entity_2), Some(&8));
        assert_eq!(storage.get(test_entity_3), Some(&2025));
        assert_eq!(storage.get(test_entity_nope), None);
    }

    #[test]
    fn test_get_mut() {
        let mut storage = ComponentStorage::<i32>::new();
        let test_entity_1 = Entity::new(1);
        let test_entity_2 = Entity::new(2);
        let test_entity_3 = Entity::new(3);
        let test_entity_nope = Entity::new(4);

        storage.insert(test_entity_1, 30);
        storage.insert(test_entity_2, 8);
        storage.insert(test_entity_3, 2025);
        assert_eq!(storage.get_mut(test_entity_1), Some(&mut 30));
        assert_eq!(storage.get_mut(test_entity_2), Some(&mut 8));
        assert_eq!(storage.get_mut(test_entity_3), Some(&mut 2025));
        assert_eq!(storage.get_mut(test_entity_nope), None);
    }

    #[test]
    fn test_remove() {
        let mut storage = ComponentStorage::<i32>::new();
        let test_entity = Entity::new(1);

        storage.insert(test_entity, 21);
        assert!(storage.storage.contains_key(&test_entity));
        storage.remove(test_entity);
        assert!(!storage.storage.contains_key(&test_entity));
    }

    #[test]
    #[should_panic(expected = "Component missmatch")]
    fn test_invalid_any_insert() {
        let mut storage = ComponentStorage::<TestComponent>::new();

        storage.insert_any(Entity::new(1), Box::new(OtherComponent{}))
    }
}
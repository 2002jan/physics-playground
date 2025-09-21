use crate::entity::Entity;
use std::any::Any;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;

pub trait ComponentStorageAny: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn remove(&mut self, entity: Entity) -> Option<Box<dyn Any>>;
    fn insert_any(&mut self, entity: Entity, component: Box<dyn Any + 'static>);
}

pub struct ComponentStorage<T: 'static> {
    storage: HashMap<Entity, Rc<RefCell<T>>>,
}

impl<T: 'static> ComponentStorageAny for ComponentStorage<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn remove(&mut self, entity: Entity) -> Option<Box<dyn Any>> {
        let component = self.storage.remove(&entity)?;
        let component = Rc::try_unwrap(component).ok()?.into_inner();

        Some(Box::new(component))
    }

    fn insert_any(&mut self, entity: Entity, component: Box<dyn Any + 'static>) {
        let component = component.downcast::<T>().expect("Component missmatch");

        self.storage.insert(
            entity,
            Rc::new(RefCell::new(
                *component
            )),
        );
    }
}

impl<T: 'static> ComponentStorage<T> {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    pub fn insert(&mut self, entity: Entity, component: T) {
        self.storage
            .insert(entity, Rc::new(RefCell::new(component)));
    }

    pub fn get(&self, entity: Entity) -> Option<Ref<'_, T>> {
        Some(self.storage.get(&entity)?.borrow())
    }

    pub fn get_mut(&self, entity: Entity) -> Option<RefMut<'_, T>> {
        Some(self.storage.get(&entity)?.borrow_mut())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::DerefMut;

    struct TestComponent {}
    struct OtherComponent {}

    struct A {
        x: u32
    }

    #[test]
    fn test_new() {
        let storage = ComponentStorage::<TestComponent>::new();

        assert!(storage.storage.is_empty());
    }

    #[test]
    fn test_insert() {
        let mut storage = ComponentStorage::<i32>::new();
        let test_entity = Entity::new(1);

        storage.insert_any(test_entity, Box::new(21));
        assert!(storage.storage.contains_key(&test_entity));
        assert_eq!(*storage.get(test_entity).unwrap(), 21);

        let mut storage = ComponentStorage::<A>::new();
        storage.insert_any(test_entity, Box::new(A {x: 15}))
    }

    #[test]
    fn test_get() {
        let mut storage = ComponentStorage::<i32>::new();
        let test_entity_1 = Entity::new(1);
        let test_entity_2 = Entity::new(2);
        let test_entity_3 = Entity::new(3);
        let test_entity_nope = Entity::new(4);

        storage.insert_any(test_entity_1, Box::new(30));
        storage.insert_any(test_entity_2, Box::new(8));
        storage.insert_any(test_entity_3, Box::new(2025));
        assert_eq!(*storage.get(test_entity_1).unwrap(), 30);
        assert_eq!(*storage.get(test_entity_2).unwrap(), 8);
        assert_eq!(*storage.get(test_entity_3).unwrap(), 2025);
        assert!(storage.get(test_entity_nope).is_none());
    }

    #[test]
    fn test_get_mut() {
        let mut storage = ComponentStorage::<i32>::new();
        let test_entity_1 = Entity::new(1);
        let test_entity_2 = Entity::new(2);
        let test_entity_3 = Entity::new(3);
        let test_entity_nope = Entity::new(4);

        storage.insert_any(test_entity_1, Box::new(30));
        storage.insert_any(test_entity_2, Box::new(8));
        storage.insert_any(test_entity_3, Box::new(2025));
        assert_eq!(storage.get_mut(test_entity_1).unwrap().deref_mut(), &mut 30);
        assert_eq!(storage.get_mut(test_entity_2).unwrap().deref_mut(), &mut 8);
        assert_eq!(storage.get_mut(test_entity_3).unwrap().deref_mut(), &mut 2025);
        assert!(storage.get_mut(test_entity_nope).is_none());
    }

    #[test]
    fn test_remove() {
        let mut storage = ComponentStorage::<i32>::new();
        let test_entity = Entity::new(1);

        storage.insert_any(test_entity, Box::new(21));
        assert!(storage.storage.contains_key(&test_entity));
        storage.remove(test_entity);
        assert!(!storage.storage.contains_key(&test_entity));
    }

    #[test]
    #[should_panic(expected = "Component missmatch")]
    fn test_invalid_any_insert() {
        let mut storage = ComponentStorage::<TestComponent>::new();

        storage.insert_any(Entity::new(1), Box::new(OtherComponent {}))
    }
}

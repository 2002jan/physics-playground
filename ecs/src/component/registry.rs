use std::collections::HashMap;
use std::any::TypeId;
use crate::component::storage::{ComponentStorage, ComponentStorageAny};

pub struct ComponentRegistry {
    storage_creation_function: HashMap<TypeId, fn() -> Box<dyn ComponentStorageAny>>
}

impl ComponentRegistry {
    pub fn new() -> Self {
        Self {
            storage_creation_function: HashMap::new(),
        }
    }

    pub fn register_component<T: 'static>(&mut self) {
        self.storage_creation_function.insert(
            TypeId::of::<T>(),
            || {Box::new(ComponentStorage::<T>::new())}
        );
    }

    pub fn create_component_storage(&self, type_id: &TypeId) -> Option<Box<dyn ComponentStorageAny>>{
        let create_fn = self.storage_creation_function.get(type_id)?;
        Some(create_fn())
    }

    pub fn is_registered<T: 'static>(&self) -> bool {
        self.is_id_registered(&TypeId::of::<T>())
    }

    pub fn is_id_registered(&self, type_id: &TypeId) -> bool {
        self.storage_creation_function.contains_key(type_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Component1 {}

    struct Component2 {}

    #[test]
    fn test_type_registration() {
        let mut component_registry = ComponentRegistry::new();

        component_registry.register_component::<Component1>();
        component_registry.register_component::<Component2>();

        assert!(component_registry.storage_creation_function.contains_key(&TypeId::of::<Component1>()));
        assert!(component_registry.storage_creation_function.contains_key(&TypeId::of::<Component2>()));
    }

    #[test]
    fn test_storage_creation() {
        let mut component_registry = ComponentRegistry::new();

        component_registry.register_component::<Component1>();
        component_registry.register_component::<Component2>();

        let type1 = TypeId::of::<Component1>();
        let type2 = TypeId::of::<Component2>();

        let _storage1 = component_registry.create_component_storage(&type1)
            .unwrap()
            .as_any()
            .downcast_ref::<ComponentStorage<Component1>>()
            .unwrap();
        let _storage2 = component_registry.create_component_storage(&type2)
            .unwrap()
            .as_any()
            .downcast_ref::<ComponentStorage<Component2>>()
            .unwrap();
    }

    #[test]
    fn test_unregistered_type() {
        let mut component_registry = ComponentRegistry::new();

        component_registry.register_component::<Component1>();

        let type2 = TypeId::of::<Component2>();

        let storage2 = component_registry.create_component_storage(&type2);

        assert!(storage2.is_none())
    }
}

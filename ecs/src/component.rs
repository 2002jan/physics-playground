use std::any::Any;
pub mod registry;
pub mod storage;
mod archetype;

pub trait Component: Any {}
impl<T: Any> Component for T {}
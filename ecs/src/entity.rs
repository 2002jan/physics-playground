#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct Entity {
    id: u32,
}

impl Entity {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}
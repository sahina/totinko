use serde::Serialize;
use uuid::Uuid;

use crate::ddd::core::EntityId;

pub trait TEntityId {
    fn id(&self) -> &EntityId;
}

pub trait TEntityName {
    fn name(&self) -> &String;
}

/// Represents anything with a name and an identifier.
#[derive(Debug, Clone, Serialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct Entity {
    id: EntityId,
    name: String,
}

impl TEntityId for Entity {
    fn id(&self) -> &EntityId {
        &self.id
    }
}

impl TEntityName for Entity {
    fn name(&self) -> &String {
        &self.name
    }
}

impl Entity {
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
        }
    }
}

pub struct UuidIdGenerator {}

impl UuidIdGenerator {
    pub fn id() -> String {
        Uuid::new_v4().to_string()
    }
}

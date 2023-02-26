use chrono::Utc;
use serde::Serialize;

use crate::ddd::core::{CommandMetadata, CommandPayload};
use crate::ddd::entity::{Entity, UuidIdGenerator};

#[derive(Debug, Clone, Serialize)]
pub struct Command {
    /// Represents command name and identity
    pub entity: Entity,
    /// Metadata of the command
    pub metadata: CommandMetadata,
    /// Contents of the command
    pub payload: CommandPayload,
    /// UTC based timestamp from unix epoch
    pub timestamp: i64,
}

pub fn new_command(name: &str, payload: CommandPayload) -> Command {
    Command {
        entity: Entity::new(UuidIdGenerator::id().as_str(), name),
        metadata: Default::default(),
        payload,
        timestamp: Utc::now().timestamp(),
    }
}

// todo: CommandBuilder to builds command with payload with simple API

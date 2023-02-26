use chrono::Utc;
use serde::Serialize;
use serde_json::Value;

use crate::ddd::context::{AR_ID_KEY, AR_NAME_KEY, AR_VERSION_KEY};
use crate::ddd::core::{AggregateId, EntityId, EventMetadata, EventPayload};
use crate::ddd::entity::{Entity, TEntityId, TEntityName, UuidIdGenerator};

pub trait TEvent: TEntityId {
    fn event_name(&self) -> &String;
    fn payload(&self) -> &EventPayload;
    fn metadata(&self) -> &EventMetadata;
    fn timestamp(&self) -> i64;
}

pub trait TAggregateEvent {
    fn event(&self) -> &Event;
    fn aggregate_name(&self) -> String;
    fn aggregate_id(&self) -> AggregateId;
    fn aggregate_version(&self) -> u32;
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct Event {
    entity: Entity,
    metadata: EventMetadata,
    payload: EventPayload,
    timestamp: i64,
}

impl Event {
    pub fn new(name: &str, payload: EventPayload, metadata: Option<EventMetadata>) -> Event {
        let mut event = Event {
            entity: Entity::new(UuidIdGenerator::id().as_str(), name),
            metadata: Default::default(),
            payload,
            timestamp: Utc::now().timestamp(),
        };

        if let Some(pair) = metadata {
            event.metadata.extend(pair);
        }

        event
    }

    pub fn add_metadata(&mut self, key: &str, value: Value) {
        self.metadata.insert(key.to_string(), value);
    }

    pub fn extend_metadata(&mut self, metadata: EventMetadata) {
        self.metadata.extend(metadata);
    }
}

impl TEvent for Event {
    fn event_name(&self) -> &String {
        self.entity.name()
    }

    fn payload(&self) -> &EventPayload {
        &self.payload
    }

    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }

    fn timestamp(&self) -> i64 {
        self.timestamp
    }
}

impl TEntityId for Event {
    fn id(&self) -> &EntityId {
        self.entity.id()
    }
}

#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub struct AggregateEvent {
    event: Event,
}

impl TEntityId for AggregateEvent {
    fn id(&self) -> &EntityId {
        self.event.id()
    }
}

impl TAggregateEvent for AggregateEvent {
    fn event(&self) -> &Event {
        &self.event
    }

    fn aggregate_name(&self) -> String {
        self.metaval_str(AR_NAME_KEY)
    }

    fn aggregate_id(&self) -> AggregateId {
        self.metaval_str(AR_ID_KEY)
    }

    fn aggregate_version(&self) -> u32 {
        self.metaval_int(AR_VERSION_KEY)
    }
}

impl AggregateEvent {
    pub fn new(event: Event) -> Self {
        AggregateEvent { event }
    }

    fn metaval_str(&self, key: &str) -> String {
        if let Some(key_value) = self.event.metadata.get(key) {
            let value: String = serde_json::from_value(key_value.clone()).unwrap();
            value
        } else {
            String::default()
        }
    }

    fn metaval_int(&self, key: &str) -> u32 {
        if let Some(key_value) = self.event.metadata.get(key) {
            let value: u32 = serde_json::from_value(key_value.clone()).unwrap();
            value
        } else {
            0
        }
    }
}

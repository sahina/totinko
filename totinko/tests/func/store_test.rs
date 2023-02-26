use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use color_eyre::eyre::Result;
use serde::Serialize;
use thiserror::Error;

use totinko::func::event::store::{EventEnvelope, EventStore};

#[derive(Debug, Error)]
pub enum EventStoreError {}

type LockedEventEnvelopeMap<E> = RwLock<HashMap<String, Vec<EventEnvelope<E>>>>;

#[derive(Debug)]
pub struct MemoryEventStore<E: Serialize + Send + Sync> {
    events: Arc<LockedEventEnvelopeMap<E>>,
}

#[async_trait]
impl<E: Serialize + Send + Sync> EventStore<E> for MemoryEventStore<E> {
    type StoreError = EventStoreError;

    async fn load(&self) -> Result<(), Self::StoreError> {
        Ok(())
    }

    async fn save(&self, events: Vec<EventEnvelope<E>>) -> Result<(), EventStoreError> {
        Ok(())
    }
}

#[test]
fn test_esdb_connection() {}

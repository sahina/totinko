use std::error::Error;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::func::event::repository::StreamId;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventEnvelope<E: Serialize> {
    pub stream_id: StreamId,
    pub stream_name: String,
    pub event: E,
    pub version: u32,
}

#[async_trait]
pub trait EventStore<E: Serialize> {
    type StoreError: Error;

    async fn load(&self) -> Result<(), Self::StoreError>;
    async fn save(&self, events: Vec<EventEnvelope<E>>) -> Result<(), Self::StoreError>;
}

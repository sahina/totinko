use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use thiserror::Error;

use crate::ddd::entity::{TEntityId, TEntityName};
use crate::ddd::event::{AggregateEvent, TAggregateEvent, TEvent};
use crate::es::aggregate_store::TAggregateStore;
use crate::es::context::ESAggregateContext;
use crate::es::event::EventCommitter;

type LockedEventEnvelopeMap = RwLock<HashMap<String, Vec<AggregateEvent>>>;

#[derive(Debug, Error)]
pub enum MemStoreErrors {}

#[derive(Debug, Default)]
pub struct MemStore {
    events: Arc<LockedEventEnvelopeMap>,
}

impl MemStore {
    pub fn new() -> Self {
        Self {
            events: Arc::new(Default::default()),
        }
    }
}

#[async_trait]
impl TAggregateStore for MemStore {
    type Error = MemStoreErrors;

    async fn load(&self, context: &mut ESAggregateContext) -> Result<(), Self::Error> {
        let event_map = self.events.read().unwrap();
        let aggregate_name = context.name().clone();

        match event_map.get(context.id()) {
            None => {}
            Some(saved_events) => {
                // todo: sort events
                for saved_event in saved_events {
                    context.add_event(
                        aggregate_name.as_str(),
                        saved_event.event().payload().clone(),
                        None,
                    );
                }
                context.commit_events();
            }
        };

        Ok(())
    }

    async fn save(&self, _context: &mut ESAggregateContext) -> Result<(), Self::Error> {
        Ok(())
    }
}

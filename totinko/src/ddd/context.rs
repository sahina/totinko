use serde::Serialize;
use serde_json::json;

use crate::ddd::aggregate::Eventer;
use crate::ddd::core::{EntityId, EventMetadata, EventPayload};
use crate::ddd::entity::{Entity, TEntityId, TEntityName};
use crate::ddd::event::AggregateEvent;
use crate::ddd::event::Event;

/// Key names for metadata updates
pub const AR_NAME_KEY: &str = "aggregate_name";
pub const AR_ID_KEY: &str = "aggregate_id";
pub const AR_VERSION_KEY: &str = "aggregate_version";

/// Context for a dddd based aggregate.
#[derive(Debug, Serialize)]
pub struct AggregateContext {
    entity: Entity,
    events: Vec<AggregateEvent>,
}

impl AggregateContext {
    pub fn new(id: &str, name: &str) -> AggregateContext {
        AggregateContext {
            entity: Entity::new(id, name),
            events: Default::default(),
        }
    }
}

impl Eventer for AggregateContext {
    fn add_event(&mut self, name: &str, payload: EventPayload, metadata: Option<EventMetadata>) {
        let mut event = Event::new(name, payload, metadata.clone());
        event.add_metadata(AR_NAME_KEY, json!(self.entity.name()));
        event.add_metadata(AR_ID_KEY, json!(self.entity.id()));

        // extend event metadata if additional is received
        if let Some(value) = metadata {
            event.extend_metadata(value);
        }

        self.events.push(AggregateEvent::new(event));
    }

    fn events(&self) -> &Vec<AggregateEvent> {
        &self.events
    }

    fn clear_events(&mut self) {
        self.events.clear();
    }
}

impl TEntityId for AggregateContext {
    fn id(&self) -> &EntityId {
        self.entity.id()
    }
}

impl TEntityName for AggregateContext {
    fn name(&self) -> &String {
        self.entity.name()
    }
}

#[cfg(test)]
mod ddd_context_tests {
    use serde_json::json;

    use crate::ddd::aggregate::Eventer;
    use crate::ddd::context::AggregateContext;
    use crate::ddd::core::EventPayload;
    use crate::ddd::entity::{TEntityId, TEntityName};
    use crate::ddd::event::TAggregateEvent;

    #[test]
    fn test_new_ddd_context_should_have_no_events() {
        let context = AggregateContext::new("1", "OrderAR");

        assert_eq!(context.events().len(), 0);
        assert_eq!(context.id(), "1");
        assert_eq!(context.name(), "OrderAR");
    }

    #[test]
    fn test_ddd_context_events() {
        let mut context = AggregateContext::new("1", "OrderAR");
        context.add_event("CreateOrder", EventPayload(json!("online order")), None);
        context.add_event("AddProduct", EventPayload(json!("product-id:123")), None);

        assert_eq!(context.events().len(), 2);
    }

    #[test]
    fn test_ddd_context_event_details() {
        let mut context = AggregateContext::new("1", "OrderAR");
        context.add_event("CreateOrder", EventPayload(json!("online order")), None);

        let aggregate_event = context.events().get(0).unwrap();

        assert_eq!(aggregate_event.aggregate_id(), "1");
        assert_eq!(aggregate_event.aggregate_version(), 0);
        assert_eq!(aggregate_event.aggregate_name(), "OrderAR");
    }
}

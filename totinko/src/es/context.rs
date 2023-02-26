use serde::Serialize;
use serde_json::json;

use crate::ddd::aggregate::Eventer;
use crate::ddd::context::{AggregateContext, AR_VERSION_KEY};
use crate::ddd::core::{EntityId, EventMetadata, EventPayload};
use crate::ddd::entity::{TEntityId, TEntityName};
use crate::es::event::{EventCommitter, VersionSetter, Versioner};

/// Context for an event sourced aggregate.
#[derive(Debug, Serialize)]
pub struct ESAggregateContext {
    aggregate: AggregateContext,
    version: u32,
}

impl ESAggregateContext {
    pub fn new(id: &str, name: &str) -> ESAggregateContext {
        ESAggregateContext {
            aggregate: AggregateContext::new(id, name),
            version: 0,
        }
    }

    pub fn add_event(
        &mut self,
        name: &str,
        payload: EventPayload,
        metadata: Option<EventMetadata>,
    ) {
        // bump aggregate version with each new added event
        let version = self.pending_version() + 1;

        let mut event_metadata = EventMetadata::new();
        event_metadata.insert(AR_VERSION_KEY.to_string(), json!(version));

        if let Some(received_metadata) = metadata {
            event_metadata.extend(received_metadata);
        }

        self.aggregate
            .add_event(name, payload, Some(event_metadata));
    }
}

impl Versioner for ESAggregateContext {
    fn version(&self) -> u32 {
        self.version
    }

    fn pending_version(&self) -> u32 {
        self.version + u32::try_from(self.aggregate.events().len()).unwrap()
    }
}

impl VersionSetter for ESAggregateContext {
    fn set_version(&mut self, version: u32) {
        self.version = version;
    }
}

impl EventCommitter for ESAggregateContext {
    fn commit_events(&mut self) {
        self.version += u32::try_from(self.aggregate.events().len()).unwrap();
        self.aggregate.clear_events();
    }
}

impl TEntityName for ESAggregateContext {
    fn name(&self) -> &String {
        self.aggregate.name()
    }
}

impl TEntityId for ESAggregateContext {
    fn id(&self) -> &EntityId {
        self.aggregate.id()
    }
}

#[cfg(test)]
mod es_context_tests {
    use serde_json::json;

    use crate::ddd::aggregate::Eventer;
    use crate::ddd::core::EventPayload;
    use crate::ddd::entity::{TEntityId, TEntityName};
    use crate::ddd::event::TAggregateEvent;
    use crate::es::context::ESAggregateContext;
    use crate::es::event::Versioner;

    #[test]
    fn test_new_es_context_should_have_no_events() {
        let context = ESAggregateContext::new("1", "OrderAR");

        assert_eq!(context.aggregate.events().len(), 0);
        assert_eq!(context.aggregate.id(), "1");
        assert_eq!(context.aggregate.name(), "OrderAR");
    }

    #[test]
    fn test_es_context_details() {
        let mut context = ESAggregateContext::new("1", "OrderAR");

        // context w/o events
        assert_eq!(context.version(), 0);
        assert_eq!(context.pending_version(), 0);

        // add events to context
        context.add_event("CreateOrder", EventPayload(json!("online order")), None);
        context.add_event("AddProduct", EventPayload(json!("product-id:123")), None);

        assert_eq!(context.version(), 0);
        assert_eq!(context.pending_version(), 2);
    }

    #[test]
    fn test_es_context_event_details() {
        let mut context = ESAggregateContext::new("1", "OrderAR");
        context.add_event("CreateOrder", EventPayload(json!("online order")), None);

        let aggregate_event = context.aggregate.events().get(0).unwrap();

        assert_eq!(aggregate_event.aggregate_id(), "1");
        assert_eq!(aggregate_event.aggregate_version(), 1);
        assert_eq!(aggregate_event.aggregate_name(), "OrderAR");
    }
}

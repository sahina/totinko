use serde_json::{json, Value};
use uuid::Uuid;

use totinko::ddd::core::EventPayload;
use totinko::ddd::entity::TEntityId;
use totinko::ddd::entity::UuidIdGenerator;
use totinko::ddd::event::{Event, TEvent};

use crate::common::helpers::user_registration;

#[test]
fn test_new_event() {
    let event_payload = user_registration();
    let event = Event::new("UserRegistered", EventPayload(json!(event_payload)), None);

    assert_eq!(event.payload().0["email"], event_payload.email);
    assert_eq!(event.metadata().len(), 0);
    assert!(event.timestamp() > 0);
}

#[test]
fn test_event_entity() {
    let event_payload = user_registration();
    let event = Event::new("UserRegistered", EventPayload(json!(event_payload)), None);

    assert_ne!(event.id(), "");
    assert_eq!(event.event_name(), "UserRegistered");
}

#[test]
fn test_new_event_with_meta() {
    let event_payload = user_registration();
    let mut event = Event::new("UserRegistered", EventPayload(json!(event_payload)), None);

    event.add_metadata("location", json!("USA"));
    event.add_metadata("state", json!("NJ"));

    assert_eq!(event.metadata().len(), 2);
    assert_eq!(
        *event.metadata().get("location").unwrap(),
        Value::String("USA".to_string())
    );
    assert_eq!(
        *event.metadata().get("state").unwrap(),
        Value::String("NJ".to_string())
    );
}

#[test]
fn test_uuid_event_id_generator() {
    let id = UuidIdGenerator::id();
    let parsed: Uuid = Uuid::parse_str(&id).unwrap();

    assert_eq!(parsed.to_string(), id);
}

use serde_json::json;

use totinko::ddd::aggregate::Eventer;
use totinko::ddd::context::AggregateContext;
use totinko::ddd::core::EventPayload;
use totinko::ddd::entity::{TEntityId, TEntityName};

use crate::common::helpers::{user_registration, user_sign_in};

#[test]
fn test_new_ar() {
    let ar = AggregateContext::new("123", "RegistrationAR");

    assert_ne!(ar.id(), "");
    assert_eq!(ar.name(), "RegistrationAR");
    assert_eq!(ar.events().len(), 0);
}

#[test]
fn test_add_event_to_ar() {
    let mut ar = AggregateContext::new("123", "RegistrationAR");

    ar.add_event(
        "UserRegistered",
        EventPayload(json!(user_registration())),
        None,
    );
    ar.add_event("UserSignedIn", EventPayload(json!(user_sign_in())), None);

    assert_eq!(ar.events().len(), 2);
}

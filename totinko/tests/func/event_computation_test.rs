use totinko::func::decider::Decider;
use totinko::func::event::computation::{EventComputation, EventComputer};

use crate::common::helpers::types::{
    order_decide, order_evolve, OrderCommand, OrderDecider, OrderEvent, OrderState, Product,
};

#[test]
fn test_event_computation() {
    let decider = OrderDecider::new(order_decide, order_evolve, OrderState::default());
    let ec = EventComputation::new(decider);

    let events = vec![
        OrderEvent::Created("123".to_string()),
        OrderEvent::ProductAdded(Product {
            product_id: "1".to_string(),
        }),
    ];

    let output = ec.compute_events(events, OrderCommand::Ship, OrderState::default());

    assert_eq!(output.len(), 1);
    assert_eq!(*output.first().unwrap(), OrderEvent::Shipped);
}

use totinko::func::decider::{Decider, DeciderBase};

use crate::common::helpers::types::{
    order_decide, order_evolve, OrderCommand, OrderDecider, OrderEvent, OrderState, Product,
};

#[test]
fn test_multiple_commands() {
    //
    // Commands

    let create = OrderCommand::Create("123".to_string());
    let _add_product = OrderCommand::AddProduct(Product {
        product_id: "xxx".to_string(),
    });
    let _ship = OrderCommand::Ship;

    //
    // Decider

    let initial_state = OrderState::default();
    let order_decider = OrderDecider::new(order_decide, order_evolve, initial_state.clone());

    //
    // Create Order

    let events = order_decider.decide(create, OrderState::default());

    assert_eq!(events.len(), 1);
    assert_eq!(
        *events.first().unwrap(),
        OrderEvent::Created("123".to_string())
    );

    let created_state = order_decider.evolve(initial_state, events.first().unwrap().clone());

    assert_eq!(created_state.order_id, "123".to_string());
}

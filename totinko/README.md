# totinko

Simple Domain Driven Design (DDD) and Command Query Responsibility Segregation (CQRS) pattern based framework.

## Pre-Alpha

**!! Do not use this crate, development is in very early stages !!**

## v1

- [ ] ddd module
    - [x] entity
        - [x] EntityNamer
        - [x] EntityIDer
    - [x] events
        - [x] EventPayload
        - [x] TEvent
        - [x] Event::new(..)
        - [x] TAggregateEvent
    - [x] AggregateContext
    - [ ] dispatcher
    - [ ] tests
- [ ] es module
    - [ ] event
    - [ ] AggregateContext
    - [ ] 
- [ ] examples
    - [ ] ddd based example
    - [ ] event sourced aggregate example building on ddd

## Planned development

- Features
    - [ ] event store
    - [ ] snapshotting
    - [ ] memory based impl
    - [ ] postgres based impl
    - [ ] saga
- tests
    - [ ] unit
    - [ ] integration
- DX
    - [ ] makefile
    - [ ] md book
    - [ ] docker / compose
    - [ ] hashistack
        - [ ] nomad
        - [ ] terraform
        - [ ] packer
        - [ ] tilt
    - [ ] UI tester

```rust
impl OrderDecider {
    pub fn compute_events(
        &self,
        events: Vec<OrderEvent>,
        command: OrderCommand,
        initial_state: OrderState,
    ) -> Vec<OrderEvent> {
        let state = events
            .into_iter()
            .fold(initial_state, |s, e| self.evolve(s, e));
        self.decide(command, state)
    }
}
```
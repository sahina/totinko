pub trait EventPublisher {}

#[derive(Default)]
pub struct EventPublish {}

impl EventPublish {
    pub fn new() -> Self {
        EventPublish {}
    }
}

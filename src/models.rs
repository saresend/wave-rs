use uuid::Uuid;
use cpal;

pub struct Node {
    id: Uuid,
}

impl Node {
    pub fn new() -> Node {
        Node { id: Uuid::new_v4() }
    }

    pub fn initialize() {
        let device = cpal::default_output_device().expect("Could not find device");
    }
}

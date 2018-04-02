extern crate wave_rs;
use std::thread;
use std::time::Duration;
#[test]
fn test_initialize() {
    let node = wave_rs::models::Node::new();
    node.initialize();
    thread::sleep(Duration::new(5, 0));
}

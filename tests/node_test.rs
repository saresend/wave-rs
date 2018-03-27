extern crate wave_rs;

#[test]
fn test_initialize() {
    let node = wave_rs::models::Node::new();
    node.initialize();
}

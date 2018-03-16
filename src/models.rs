use traits::WaveNode;

pub struct Node {
    id: i32,
}

impl WaveNode for Node {

    type Node;

    fn new() -> Node {
        Node { id: 10 }
    }

}

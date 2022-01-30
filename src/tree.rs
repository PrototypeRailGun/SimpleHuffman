use std::{
    cmp::{Ord, Ordering, PartialOrd},
    collections::BinaryHeap,
};

#[derive(PartialEq, Eq)]
pub enum NodeType {
    Internal(Box<Node>, Box<Node>),
    Leaf(u8),
}

#[derive(PartialEq, Eq)]
pub struct Node {
    pub frequency: usize,
    pub node_type: NodeType,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.frequency.cmp(&self.frequency)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Node {
    pub fn new(frequency: usize, node_type: NodeType) -> Node {
        Node {
            frequency,
            node_type,
        }
    }
}

pub fn build_tree(frequencies: &[usize; 256]) -> Node {
    let mut heap: BinaryHeap<Node> = (0..256)
        .map(|i| Node::new(frequencies[i], NodeType::Leaf(i as u8)))
        .collect();

    while heap.len() > 1 {
        let left_child = heap.pop().unwrap();
        let right_child = heap.pop().unwrap();
        heap.push(Node::new(
            left_child.frequency + right_child.frequency,
            NodeType::Internal(Box::new(left_child), Box::new(right_child)),
        ))
    }

    heap.pop().unwrap()
}

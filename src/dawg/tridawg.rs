use serde::{Deserialize, Serialize};

use crate::node::node::Node;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriDawg {
    pub parent: Node,
    pub letter: String,
    pub child: Node,
}

impl TriDawg {
    pub fn new(parent: Node, letter: String, child: Node) -> Self {
        Self {
            parent,
            letter,
            child,
        }
    }
}

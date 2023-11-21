use serde::{Deserialize, Serialize};

use crate::node::node::Node;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriDawg {
    /// A node that extends to this node (parent)
    pub parent: Node,
    /// the letter on this node
    pub letter: String,
    /// The current node itself
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

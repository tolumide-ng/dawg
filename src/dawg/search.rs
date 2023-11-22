use serde::{Serialize, Deserialize};

use crate::node::node::Node;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub node: Node,
    pub word: String,
}

impl SearchResult {
    pub fn new(node: Node, word: String) -> Self {
        Self { node, word }
    }
}

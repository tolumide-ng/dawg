use serde::{Serialize, Deserialize};

use crate::node::node::Node;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SearchResult {
    pub(crate) node: Node,
    pub(crate) word: String,
}

impl SearchResult {
    pub(crate) fn new(node: Node, word: String) -> Self {
        Self { node, word }
    }
}

use crate::node::node::Node;

use std::borrow::Cow;

pub enum SearchReq {
    Vertex,
    Word,
}

#[derive(Debug)]
pub struct SearchRes {
    pub node: Node,
    pub word: Vec<String>,
}

impl SearchRes {
    pub fn new(node: Node, word: Vec<String>) -> Self {
        Self { node, word }
    }
}

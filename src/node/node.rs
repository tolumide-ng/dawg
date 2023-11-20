#[cfg(not(feature = "threading"))]
use std::{cell::RefCell, rc::Rc};
#[cfg(feature = "threading")]
use std::sync::{Arc, Mutex};

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[cfg(test)]
#[path = "./node.test.rs"]
mod node_test;

#[cfg(not(feature = "threading"))]
pub type Node = Rc<RefCell<DawgNode>>;
#[cfg(feature = "threading")]
pub type Node = Arc<Mutex<DawgNode>>;



/// `Node`: Represents a letter in the DAWG,
#[derive(Debug, Serialize, Deserialize)]
pub struct DawgNode {
    id: usize,
    /// specifies whether this node is the end of a valid `WORD` in your dictionary
    /// TRUE: Yes, it is the end of a valid word
    /// FALSE: No, it is not the end of a valid word
    pub terminal: bool,
    /// Letters(nodes) that extend from this letter (node)
    pub edges: HashMap<String, Node>,
    /// Specifies the total number of word terminals resulting from this node,
    /// this word terminals (letters that end a word) can be direct children, grand-children, 
    /// or even great-grand-children of this node
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DawgWrapper {
    next_id: usize,
}

impl DawgWrapper {
    pub fn new() -> Self {
        Self { next_id: 0 }
    }

    pub fn create(&mut self) -> Node {
        let node = DawgNode::new(self.next_id);
        self.next_id += 1;

        #[cfg(not(feature = "threading"))]
        return Rc::new(RefCell::new(node));
        #[cfg(feature = "threading")]
        return Arc::new(Mutex::new(node));
    }
}

impl DawgNode {
    /// Creates a new node (DawgNode)
    pub fn new(id: usize) -> Self {
        Self {
            id,
            terminal: false,
            edges: HashMap::new(),
            count: 0,
        }
    }

    /// Returns the total number of word terminals that result(are extended) from this node
    /// this can be chidlren/grand-children/great-grand-children e.t.c
    pub(crate) fn num_reachable(&mut self) -> usize {
        if self.count != 0 {
            return self.count;
        }

        let mut count = 0;

        if self.terminal {
            count += 1;
        }

        for (_, value) in &mut self.edges {        
            #[cfg(not(feature = "threading"))]
            {count += value.borrow_mut().num_reachable();}

            #[cfg(feature = "threading")]
            if let Ok(mut handle) = value.lock() {
                count += handle.num_reachable();
            }
        }

        self.count = count;
        return count;
    }

    pub fn edge_keys(&self) -> Vec<&String> {
        let keys = self.edges.keys().collect::<Vec<_>>();
        keys
    }
}

impl Display for DawgNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut arr = vec![];

        if self.terminal {
            arr.push("1".to_string());
        } else {
            arr.push("0".to_string());
        }

        for (key, value) in &self.edges {
            #[cfg(not(feature = "threading"))]
            let id = value.try_borrow().unwrap().id.to_string();

            #[cfg(feature = "threading")]
            let id = value.lock().unwrap().id.to_string();

            arr.push(id);
            arr.push(key.to_string())
        }

        let name = arr.join("_");

        write!(f, "{}", name)
    }
}

impl Eq for DawgNode {}

impl Ord for DawgNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_string().cmp(&other.to_string())
    }
}

impl PartialOrd for DawgNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for DawgNode {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

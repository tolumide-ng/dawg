use std::cmp;
use std::collections::HashMap;

#[cfg(not(feature = "threading"))]
use std::rc::Rc;
#[cfg(feature = "threading")]
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::node::node::{DawgWrapper, Node};
use crate::dawg::search::{SearchReq, SearchRes};
use crate::dawg::tridawg::TriDawg;

#[cfg(test)]
#[path = "./dawg.test.rs"]
mod dawg_test;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dawg {
    root: Node,
    minimized_nodes: HashMap<String, Node>,
    unchecked_nodes: Vec<TriDawg>,
    previous_word: String,
    node: DawgWrapper,
}

impl Dawg {
    pub fn new() -> Self {
        let mut dawg_wrapper = DawgWrapper::new();

        Self {
            root: dawg_wrapper.create(),
            minimized_nodes: HashMap::new(),
            unchecked_nodes: vec![],
            previous_word: String::new(),
            node: dawg_wrapper,
        }
    }

    fn minimize(&mut self, down_to: usize) {
        // 4 of length 8
        let mut start = self.unchecked_nodes.len() as i8 - 1;
        let end = down_to as i8 - 1;

        while start > end {
            let i = start as usize;
            let TriDawg {
                parent,
                letter,
                // rename child to current
                child,
            } = &mut self.unchecked_nodes[i];

            #[cfg(not(feature = "threading"))]
            let node = child.try_borrow().unwrap().to_string();

            #[cfg(feature = "threading")]
            let node = child.lock().unwrap().to_string();

            let exists = self.minimized_nodes.contains_key(node.as_str());

            if exists {
                let minimized_reference = self.minimized_nodes.get(node.as_str()).unwrap();

                #[cfg(not(feature = "threading"))]
                parent
                    .as_ref()
                    .borrow_mut()
                    .edges
                    .insert(letter.to_owned(), Rc::clone(minimized_reference));

                #[cfg(feature = "threading")]
                parent
                    .as_ref()
                    .lock()
                    .unwrap()
                    .edges
                    .insert(letter.to_owned(), Arc::clone(minimized_reference));
            } else {
                #[cfg(not(feature = "threading"))]
                self.minimized_nodes.insert(node, Rc::clone(child));

                #[cfg(feature = "threading")]
                self.minimized_nodes.insert(node, Arc::clone(&child));
            }

            self.unchecked_nodes.pop();

            start -= 1;
        }
    }

    pub fn insert(&mut self, word: String) {
        if self.previous_word > word {
            panic!("Error: Please ensure all words are sorted before adding")
        }

        let mut common_prefix = 0;

        let word_vec = word
            .split_terminator("")
            .skip(1)
            .collect::<Vec<_>>()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        let prev_word_vec = self
            .previous_word
            .split_terminator("")
            .skip(1)
            .collect::<Vec<_>>()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();

        let min_length = cmp::min(word_vec.len(), prev_word_vec.len());

        for index in 0..min_length {
            if word_vec[index] != prev_word_vec[index] {
                break;
            }
            common_prefix += 1;
        }

        // write out what this line does for easy onboarding
        self.minimize(common_prefix);

        // Get the letters that are not similiar to the last
        for index in common_prefix..word_vec.len() {
            let letter = word_vec[index].to_owned();

            let mut node = &self.root;

            if self.unchecked_nodes.len() != 0 {
                let last = self.unchecked_nodes.len() - 1;
                node = &self.unchecked_nodes[last].child;
            }

            let next_node = self.node.create();
            // reference the previous node (ether in the uncheckd_nodes or the root(incase it is the first))
            #[cfg(not(feature = "threading"))]
            node.as_ref()
                .borrow_mut()
                .edges
                .insert(letter.to_owned(), Rc::clone(&next_node));

            #[cfg(feature = "threading")]
            node.as_ref()
                .lock()
                .unwrap()
                .edges
                .insert(letter.to_owned(), Arc::clone(&next_node));

            // tridawg is the parent == node, and the letter + next_node == current+node
            #[cfg(not(feature = "threading"))]
            let tridawg = TriDawg::new(Rc::clone(node), letter, Rc::clone(&next_node));

            #[cfg(feature = "threading")]
            let tridawg = TriDawg::new(Arc::clone(node), letter, Arc::clone(&next_node));

            self.unchecked_nodes.push(tridawg);
        }

        let last_unchecked = self.unchecked_nodes.len() - 1;

        #[cfg(not(feature = "threading"))]
        let node = &mut self.unchecked_nodes[last_unchecked]
            .child
            .as_ref()
            .borrow_mut();

        #[cfg(feature = "threading")]
        let node = &mut self.unchecked_nodes[last_unchecked]
            .child
            .as_ref()
            .lock()
            .unwrap();

        node.terminal = true;

        self.previous_word = word;
    }

    pub fn finish(&mut self) {
        self.minimize(0);

        #[cfg(not(feature = "threading"))]
        self.root.as_ref().borrow_mut().num_reachable();

        #[cfg(feature = "threading")]
        self.root.as_ref().lock().unwrap().num_reachable();

        self.minimized_nodes = HashMap::new();
        self.unchecked_nodes = vec![];
    }

    fn find<'a>(
        &self,
        word: Vec<String>,
        return_type: SearchReq,
        case_sensitive: bool,
    ) -> Option<SearchRes> {
        #[cfg(not(feature = "threading"))]
        let mut node: Node = Rc::clone(&self.root);

        #[cfg(feature = "threading")]
        let mut node: Node = Arc::clone(&self.root);

        for i in 0..word.len() {
            let letter = word[i].to_owned();

            #[cfg(not(feature = "threading"))]
            let keys = node
                .as_ref()
                .borrow()
                .edges
                .keys()
                .collect::<Vec<_>>()
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>();

            #[cfg(feature = "threading")]
            let keys = node
                .lock()
                .unwrap()
                .edges
                .keys()
                .collect::<Vec<_>>()
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>();

            match case_sensitive {
                true => {
                    if keys.contains(&letter) {
                        #[cfg(not(feature = "threading"))]
                        let next_node = Rc::clone(&node.as_ref().borrow().edges[&letter]);

                        #[cfg(feature = "threading")]
                        let next_node = Arc::clone(&node.lock().unwrap().edges[&letter]);

                        node = next_node;
                    } else {
                        return None;
                    }
                }
                false => {
                    let keys = keys.iter().map(|x| x.to_uppercase()).collect::<Vec<_>>();

                    let letter = letter.to_uppercase();

                    if keys.contains(&letter) {
                        #[cfg(not(feature = "threading"))]
                        let next_node = Rc::clone(&node.as_ref().borrow().edges[&letter]);

                        #[cfg(feature = "threading")]
                        let next_node = Arc::clone(&node.as_ref().lock().unwrap().edges[&letter]);

                        node = next_node;
                    } else {
                        return None;
                    }
                }
            }
        }

        return Some(SearchRes::new(node, word));
    }

    /// Given a specific word, check if the word exists in the lexicon (Allowing search to be case sensitive or insensitive)
    /// TODO: WE SHOULD BE ABLE TO ACCEPT A VECTOR OF STRINGS TOO SO WE DON'T MAKE A MISTAKE IN OUR SPLITTING HERE
    /// SO SOMETHING LIKE vec!["H", "U", "M", "A", "N"]
    /// although this thinking doesn't hold up when you consider the fact that we actually split the words ourselve to build the dawg
    pub fn is_word<'a>(&self, word: Vec<String>, case_sensitive: bool) -> Option<Vec<String>> {
        let result = self.find(word, SearchReq::Word, case_sensitive);

        if let Some(context) = result {
            #[cfg(not(feature = "threading"))]
            let is_terminal = context.node.as_ref().borrow().terminal;

            #[cfg(feature = "threading")]
            let is_terminal = context.node.lock().unwrap().terminal;

            if is_terminal {
                return Some(context.word);
            }
        }

        return None;
    }

    pub fn get_root(&self) -> Node {
        #[cfg(not(feature = "threading"))]
        return Rc::clone(&self.root);

        #[cfg(feature = "threading")]
        return Arc::clone(&self.root);
    }

    /// find out if word is a prefix of anything in the dictionary
    pub fn lookup<'a>(&self, word: Vec<String>, case_sensitive: bool) -> Option<Node> {
        let result = self.find(word, SearchReq::Vertex, case_sensitive);

        if let Some(context) = result {
            return Some(context.node);
        }

        return None;
    }
}

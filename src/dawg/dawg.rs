use std::{collections::HashMap, cmp};

#[cfg(not(feature = "threading"))]
use std::rc::Rc;
#[cfg(feature = "threading")]
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::node::node::{DawgWrapper, Node};
use crate::dawg::search::SearchRes;
use crate::dawg::tridawg::TriDawg;


#[cfg(test)]
#[path = "./dawg.test.rs"]
mod dawg_test;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dawg {
    /// The root of the dawg
    root: Node,
    /// The wrapper of the dawg (generates a new id for every new dawg node) (review this comment please)
    node: DawgWrapper,
    minimized_nodes: HashMap<String, Node>,
    unchecked_nodes: Vec<TriDawg>,
    previous_word: String, // ??? Did I mean previous letter?
}

impl Dawg {
    pub fn new() -> Self {
        let mut dawg_wrapper = DawgWrapper::new();

        Self {
            root: dawg_wrapper.create(),
            node: dawg_wrapper,
            minimized_nodes: HashMap::new(),
            unchecked_nodes: vec![],
            previous_word: String::new(),
        }
    }

    /// Removes nodes in the unchecked nodes up to (down_to) e.g. 
    /// if there are 7 items in unchecked_nodes and `down_to` is 4,
    /// minimize would remove node 7, 6, and 5
    fn minimize(&mut self, down_to: usize) {
        let mut start = self.unchecked_nodes.len() as i8 - 1;
        let end = down_to as i8 - 1;

        while start > end {
            let index = start as usize;
            
            let TriDawg {
                parent,
                letter,
                // rename child to current
                child: current,
            } = &mut self.unchecked_nodes[index];

            #[cfg(not(feature = "threading"))]
            let node = current.try_borrow().unwrap().to_string();
            #[cfg(feature = "threading")]
            let node = current.lock().unwrap().to_string();

            let exists = self.minimized_nodes.contains_key(node.as_str());

            // if the current node already exists in our minimize nodes list, map the parent to the existing node rather than creating a new one with current
            if exists {
                let minimized_reference = self.minimized_nodes.get(node.as_str()).unwrap();


                // same letter but updates to the connection to an already existing node in the dawg (minimized nodes)
                #[cfg(not(feature = "threading"))]
                parent.as_ref().borrow_mut().edges.insert(letter.to_owned(), Rc::clone(minimized_reference));
                #[cfg(feature = "threading")]
                parent.as_ref().lock().unwrap().edges.insert(letter.to_owned(), Arc::clone(minimized_reference));
            } else {
                #[cfg(not(feature = "threading"))]
                self.minimized_nodes.insert(node, Rc::clone(current));
                #[cfg(feature = "threading")]
                self.minimized_nodes.insert(node, Arc::clone(&current));
            }

            self.unchecked_nodes.pop();
            
            start -= 1;
        }
    }


    /// Adds a word into our Dawg
    /// Panics if the word you're trying to insert is lesser than a previously inserted one
    /// ** words are expected to have been sorted (alphabetical order) before insertion into the dawg
    pub fn insert(&mut self, word: String) {
        if self.previous_word > word {
            panic!("Error: Please ensure all words are sorted before adding")
        }

        let mut common_prefix = 0;

        let word_vec = word.split_terminator("").skip(1).map(|l| l.to_string()).collect::<Vec<String>>();
        let prev_word_vec = self.previous_word.split_terminator("").skip(1).map(|l| l.to_string()).collect::<Vec<String>>();

        let min_length = cmp::min(word_vec.len(), prev_word_vec.len());

        for index in 0..min_length {
            if word_vec[index] != prev_word_vec[index] {
                break;
            }
            common_prefix += 1;
        }

        // write out what this line does for easy onboarding
        self.minimize(common_prefix);

        // Get the remaining letters that are not a part of the common prefix
        for index in common_prefix..word_vec.len() {
            let letter = word_vec[index].to_owned();

            // having established the common prefixes earlier (which we won't be duplicating)
            // we would extend the last node with the remaining letters from our new word
            let mut parent = &self.root;

            // if the unchecked nodes vec is not empty, then use the last node in it
            if let Some(last_node) = self.unchecked_nodes.last() {
                parent = &last_node.child;
            }


            let current = self.node.create();
            // reference the previous node (either in the uncheckd_nodes or the root(incase it is the first))
            #[cfg(not(feature = "threading"))]
            parent.as_ref().borrow_mut().edges.insert(letter.to_owned(), Rc::clone(&current));
            #[cfg(feature = "threading")]
            parent.as_ref().lock().unwrap().edges.insert(letter.to_owned(), Arc::clone(&current));

            // tridawg is the parent == node
            #[cfg(not(feature = "threading"))]
            let tridawg = TriDawg::new(Rc::clone(parent), letter, Rc::clone(&current));
            #[cfg(feature = "threading")]
            let tridawg = TriDawg::new(Arc::clone(parent), letter, Arc::clone(&current));

            self.unchecked_nodes.push(tridawg);
        }

        let last_node = self.unchecked_nodes.last().unwrap();
        
        #[cfg(not(feature = "threading"))]
        {last_node.child.as_ref().borrow_mut().terminal = true};
        #[cfg(feature = "threading")]
        {last_node.child.as_ref().lock().unwrap().terminal = true};

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
        self.previous_word = String::new();
    }

    fn find<'a>(
        &self,
        word: Vec<String>,
        case_sensitive: bool,
    ) -> Option<SearchRes> {
        
        #[cfg(not(feature = "threading"))]
        let mut node: Node = Rc::clone(&self.root);
        #[cfg(feature = "threading")]
        let mut node: Node = Arc::clone(&self.root);

        for i in 0..word.len() {
            let letter = word[i].to_owned();

            #[cfg(not(feature = "threading"))]
            let keys = node.borrow().edges.keys().map(|x| x.to_string()).collect::<Vec<_>>();
            #[cfg(feature = "threading")]
            let keys = node.lock().unwrap().edges.keys().map(|x| x.to_string()).collect::<Vec<_>>();

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
        let result = self.find(word, case_sensitive);

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
        let result = self.find(word, case_sensitive);

        if let Some(context) = result {
            return Some(context.node);
        }

        return None;
    }
}

use std::collections::HashSet;
use std::{collections::HashMap, cmp};

#[cfg(not(feature = "threading"))]
use std::rc::Rc;
#[cfg(feature = "threading")]
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;

use crate::node::node::{DawgWrapper, Node};
use crate::dawg::search::SearchResult;
use crate::dawg::tridawg::TriDawg;


#[cfg(test)]
#[path = "./dawg.test.rs"]
mod dawg_test;

#[derive(Debug, Clone, Deserialize, Serialize)]
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
        let unchecked_nodes = self.unchecked_nodes.len();
        if unchecked_nodes == 0 { return }        
        let mut start = unchecked_nodes - 1;
        let end = down_to;

        while start >= end {
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

            if start == 0 { break; } // handle underflow
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

        
        let word_vec = word.graphemes(true).collect::<Vec<_>>();
        let prev_word_vec = self.previous_word.graphemes(true).collect::<Vec<_>>();

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

    /// Closes the dawg after all words have been inserted into it
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

    fn find<'a>(&self, word: impl AsRef<str>, case_sensitive: bool) -> Option<SearchResult> {
        let letters = word.as_ref().graphemes(true).collect::<Vec<_>>();
        
        #[cfg(not(feature = "threading"))]
        let mut node: Node = Rc::clone(&self.root);
        #[cfg(feature = "threading")]
        let mut node: Node = Arc::clone(&self.root);

        for i in 0..letters.len() {
            let letter = letters[i].to_owned();

            #[cfg(not(feature = "threading"))]
            let keys = node.borrow().edges.keys().map(|x| x.to_owned()).collect::<Vec<_>>();
            #[cfg(feature = "threading")]
            let keys = node.lock().unwrap().edges.keys().map(|x| x.to_owned()).collect::<Vec<_>>();

            match case_sensitive {
                true => {
                    if keys.contains(&&letter) {
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
                    let letter = letter.to_uppercase();
                    let letter_exists = keys.iter().position(|x| x.to_uppercase() == letter);

                    if let Some(index) = letter_exists {
                        let actual_letter = &keys[index];
                        #[cfg(not(feature = "threading"))]
                        let next_node = Rc::clone(&node.as_ref().borrow().edges[actual_letter]);
                        #[cfg(feature = "threading")]
                        let next_node = Arc::clone(&node.as_ref().lock().unwrap().edges[&actual_letter]);

                        node = next_node;
                    } else {
                        return None;
                    }
                }
            }
        }

        return Some(SearchResult::new(node, word.as_ref().to_owned()));
    }

    /// Given a specific word, check if the word exists in the lexicon (Allowing search to be case sensitive or insensitive)
    /// 
    /// ```rust
    /// use dawg::Dawg;
    /// let mut words = vec!["SCHIST", "TILS", "LISTEN", "STIL", "SILLY", "SILENT", "CAREER", "BEAUTIFUL", "SUCCESS"];
    /// words.sort();
    /// 
    /// let mut lexicon = Dawg::new();
    /// 
    /// for word in words {
    ///     lexicon.insert(word.to_string());
    /// }
    /// 
    /// lexicon.finish(); // always remember to close the dawg when you're done inserting;
    /// // check  if "SILLY" is a valid word
    /// let result = lexicon.is_word("SILLY".to_string(), true);
    /// 
    /// assert!(result.is_some());
    /// 
    /// ```
    /// // assert!(result.is_some());
    pub fn is_word<'a>(&self, word: impl AsRef<str>, case_sensitive: bool) -> Option<String> {
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

    /// Returns the root node of the dawgie
    pub fn get_root(&self) -> Node {
        #[cfg(not(feature = "threading"))]
        return Rc::clone(&self.root);

        #[cfg(feature = "threading")]
        return Arc::clone(&self.root);
    }

    /// find out if word is a prefix of anything in the dictionary
    pub fn lookup<'a>(&self, word: impl AsRef<str>, case_sensitive: bool) -> Option<Node> {
        let result = self.find(word, case_sensitive);

        if let Some(context) = result {
            return Some(context.node);
        }

        return None;
    }


    /// this search is case sensitive
    fn anagrams(&self, current: String, remaining: &Vec<&str>) -> Vec<String> {
        if remaining.is_empty() {
            if let Some(formed) = self.is_word(&current, true) {
                return vec![formed];
            } else {
                return Vec::with_capacity(0)
            };
        }

        let mut words: HashSet<String> = HashSet::new();

        
        for (index, letter) in remaining.iter().enumerate() {
            let mut received = remaining.to_vec();
            
            let possible = format!("{}{}", current, letter);

            // remove the letter
            let _ = &received.remove(index);

            let result = self.anagrams(possible, &received);
            words.extend(result);
        }


        words.into_iter().collect()
    }

    /// Gets all valid anagrams of the word provided
    /// e.g "ATE" would return vec!["EAT", "TEA", "ATE"] asumming the dictionary you loaded the dawg with contains all these words
    pub fn find_anagrams(&self, word: impl AsRef<str>) -> Vec<String> {

        let letters = word.as_ref().graphemes(true).collect::<Vec<_>>();
        let result = self.anagrams(String::from(""), &letters);

        return result;
    }


    /// Extends a provided prefix (`extend`) to the right using the letters you provided
    /// e.g given "PICK" as extend and "YEDTUREI" as the letters, this function would return results like
    fn word_generator(&self, extend: impl AsRef<str>, letters: &Vec<&str>) -> Vec<String> {
        let mut words: HashSet<String> = HashSet::new();

        if let Some(word) = self.find(&extend, true) {
            #[cfg(feature = "threading")]
            let is_terminal = word.node.lock().unwrap().terminal;
            #[cfg(not(feature = "threading"))]
            let is_terminal = word.node.borrow().terminal;
            
            if is_terminal {
                words.insert(word.word);
            }
        }

        if letters.is_empty() {
            return words.into_iter().collect();
        }

        for i in 0..letters.len() {
            let mut local_letters = letters.to_owned();
            let letter = local_letters[i];
            local_letters.remove(i);

            let possible_word = format!("{}{}", &extend.as_ref(), letter);
            
            let result = self.word_generator(&possible_word, &local_letters);
            words.extend(result);
        }

        words.into_iter().collect()
    }


    /// (Highly inefficient approach: O(n!))
    /// This is a really expensive/brute-force approach, if you have a better suggestion, please reach out or raise a PR
    /// Returns all the possible combination of words that can be formed when the provided `extend` variable is extended either to the right or left
    /// e.g. given "IST" as prefix and ""
    /// ```rust
    /// use dawg::Dawg;
    /// 
    /// let mut words = vec!["SCHIST", "TILS", "LISTEN", "STIL", "SILLY", "SILENT", "CAREER", "BEAUTIFUL", "SUCCESS"];
    /// words.sort();
    /// 
    /// let mut lexicon = Dawg::new();
    /// 
    /// for word in words {
    ///     lexicon.insert(word.to_string());
    /// }
    /// 
    /// lexicon.finish(); // always remember to close the dawg when you're done inserting;
    /// 
    /// // assuming we want to see all the possible ways to extend "IST" with the letters "SENTILLCH";
    /// let mut result = lexicon.extend_with("IST", "LHENSC"); // would return vec!["SCHIST", "LISTEN"]
    /// result.sort();
    /// 
    /// let expected = vec!["LISTEN", "SCHIST"];
    /// assert_eq!(result, expected);
    /// ```
    pub fn extend_with(&self, extend: impl AsRef<str>, with: impl AsRef<str>) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let combined = format!("{}{}", with.as_ref(), extend.as_ref());
        let letters = combined.graphemes(true).collect::<Vec<_>>();

        let words = self.word_generator("", &letters);

        if extend.as_ref().len() > 0 {
            for word in words {
                if word.contains(&extend.as_ref()) {
                    result.push(word);
                }
            }
        } else {
            result = words;
        }
        
        return result;

    }
}

use std::{borrow::Cow, cmp, collections::HashMap};

use crate::repository::Repository;
use sqlx::{Pool, Postgres};

use crate::node::sqlx_node::{NodeEdge, SqlNode};

#[derive(Debug, Clone)]
pub struct SqlDawg {
    // consider making this the id of the root node instead
    root: SqlNode,
    // an hashmap of the key: .to_string() of the node and value: as the id of the node
    // this means that this id, has the aggregation of the ids(String) as its edges
    /// (i64, String) where i64 = id of the node, and String is the corresponding letter of this node
    minimized_nodes: HashMap<String, (i64, String)>,
    // A vector of the id(s) of the unchecked nodes
    unchecked_nodes: Vec<NodeEdge>,
    previous_word: String,
    // wrapper: SqlDawgWrapper,
    pool: Pool<Postgres>,
}

#[derive(Debug, Clone, Copy)]
pub struct SqlSearchRes {
    pub node: i64,
    pub terminal: bool,
}

impl SqlSearchRes {
    pub fn new(node: i64, terminal: bool) -> Self {
        Self { node, terminal }
    }
}

impl SqlDawg {
    pub async fn new(pool: Pool<Postgres>) -> Self {
        let root = Repository::create_node(&pool, String::from("@"), None, false).await;

        let root = root.unwrap();

        Self {
            root,
            minimized_nodes: HashMap::new(),
            previous_word: String::new(),
            unchecked_nodes: vec![],
            pool,
        }
    }

    /// Initialize an already existing dawg
    /// This should only be called only if there is an already existing dawg else, it would panic
    pub async fn init(pool: Pool<Postgres>) -> Self {
        let root = Repository::find_node(&pool, String::from("@"))
            .await
            .unwrap()
            .unwrap();

        Self {
            root,
            minimized_nodes: HashMap::new(),
            previous_word: String::new(),
            unchecked_nodes: vec![],
            pool,
        }
    }

    /// minimize the unchecked knowed from: usize
    /// where from: is the common prefix between the last word in the unchecked_nodes vector and the newly added word on the (insert method)
    pub async fn minimize(&mut self, from: usize) {
        if self.unchecked_nodes.len() == 0 {
            return;
        }

        let remove = &self.unchecked_nodes[from..]
            .iter()
            .map(|x| x.edge_id)
            .collect::<Vec<i64>>();

        let unchecked_nodes_with_edges = Repository::get_nodes_with_edges(&self.pool, remove).await;

        if let Ok(nodes) = unchecked_nodes_with_edges {
            for index in (from..self.unchecked_nodes.len()).rev() {
                let node_index = index - from;

                let node = &nodes[node_index];
                // let node_letter = &node.letter;
                let node_str = &node.to_string();

                // is there an existing node on minmized nodes that has the exact same parents as this node?
                let exists = self.minimized_nodes.contains_key(node_str);

                if exists {
                    let (edge_id, letter) = self.minimized_nodes.get(node_str).unwrap();
                    let parent_id = &self.unchecked_nodes[index];

                    // add the node as a child of this parent_id
                    Repository::add_parent(
                        &self.pool,
                        &NodeEdge::new(parent_id.node_id, *edge_id, letter.into()),
                    )
                    .await
                    .unwrap();
                } else {
                    self.minimized_nodes
                        .insert(node_str.to_owned(), (node.id, node.letter.clone()));
                }

                self.unchecked_nodes.pop();
            }
        }
    }

    pub async fn insert(&mut self, word: String) {
        if self.previous_word > word {
            panic!("Error: Please ensure all words are sorted before adding");
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

        self.minimize(common_prefix).await;

        // this is not yet as optimized as I would like it to be
        for index in common_prefix..word_vec.len() {
            let letter = word_vec[index].to_owned();

            // node is either the root or the unchecked nodes(when they exist) - this would be the parent of the current letter
            let mut node_id = &self.root.id;

            // can we move the unchecked nodes to postgres?
            // should we create a new table for unchecked_nodes or demacate the somehow on the general table = NO
            if self.unchecked_nodes.len() != 0 {
                let last = self.unchecked_nodes.len() - 1;
                node_id = &self.unchecked_nodes[last].edge_id;
            }

            let last_letter_in_word = index == word_vec.len() - 1;

            let is_terminal = if last_letter_in_word { true } else { false };

            // Creates a new_node and append it to the `node` table
            let next_node =
                Repository::create_node(&self.pool, letter, Some(*node_id), is_terminal).await;

            if let Ok(new_node) = next_node {
                self.unchecked_nodes
                    .push(NodeEdge::new(*node_id, new_node.id, new_node.letter));
            } else {
                // handle error: Retry twice/thrice, or else crash entrire program to restart afresh
            }
        }

        self.previous_word = word;
    }

    /// this only check if we have a word with that length or more
    /// Returns a vector of each letter in the "word" alonside whether such letter is a terminal or not
    async fn find(&self, word: String, case_sensitive: bool) -> Option<Vec<SqlSearchRes>> {
        // this rides on the assumption that the dictionary is all in uppercase
        let real_word = if case_sensitive {
            word
        } else {
            word.to_uppercase()
        };

        let word_vec = real_word
            .split_terminator("")
            .skip(1)
            .collect::<Vec<_>>()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();

        // we can do the find here after we've confirmed the return type of our is_word from Respository struct
        let word_check = Repository::is_word(&self.pool, self.root.id, &word_vec)
            .await
            .unwrap();

        if word_check.len() == word_vec.len() {
            return Some(word_check);
        }

        return None;
    }

    pub async fn finish(&mut self) {
        self.minimize(0).await;
        self.root.num_reachable().await;
        self.minimized_nodes = HashMap::new();
        self.unchecked_nodes = vec![];
    }

    /// checks if the provied word is valid
    /// uses the find method internally, and returns Some(word) if the word exists and is valid
    pub async fn is_word<'a>(
        &self,
        word: Cow<'a, str>,
        case_sensitive: bool,
    ) -> Option<Cow<'a, str>> {
        let result = self.find(word.clone().into_owned(), case_sensitive).await;

        if let Some(search_result) = result {
            let same_word = search_result.len() == word.len();
            let valid_word = search_result[search_result.len() - 1].terminal;

            if same_word && valid_word {
                return Some(word);
            }
        }

        return None;
    }

    pub async fn lookup<'a>(&self, word: Cow<'a, str>, case_sensitive: bool) -> Option<SqlNode> {
        let result = self.find(word.to_string(), case_sensitive).await;

        let word_vec = word
            .split_terminator("")
            .skip(1)
            .collect::<Vec<_>>()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();

        if let Some(context) = result {
            if word.len() == context.len() {
                let SqlSearchRes { node, terminal } = context[context.len() - 1];

                let letter = &word_vec[word_vec.len() - 1];

                let node = SqlNode::new(node, letter.to_owned(), terminal);

                return Some(node);
            }
        }

        None
    }
}

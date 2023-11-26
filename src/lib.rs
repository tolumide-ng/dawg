//! Implementation of Directed Acyclic Word Graph (DAWG) in Rust (pronounced "DAWG") as described by 
//! Steve Hanov [Compressing Dictionaries with a DAWG](http://stevehanov.ca/blog/?id=115) (thank you!!)
//! 
//! 
//! 
//! Add the following to your `Cargo.toml`
//! 
//! ```toml
//! [depedencies.dawg]
//! version = "x"
//! features = ["threading" ]
//! ```
//! [threading] - Support Send + Sync
//! 
//! ```Rust
//! use dawg::Dawg;
//! 
//! let mut dawgie = Dawg::new();
//! let mut words = vec!["BAM", "BAT", "BATH", "CATH", "BATHE", "CAR", "CARS", "CAREERS, "SILENT", "LIST", "LISTEN", "AYÒ", "ÒYÀ"].iter().map(|w| w.to_string().to_uppercase()).collect::<Vec<_>>();
//! 
//! words.sort();
//! 
//! for word in words {
//!     dawgie.insert(word.to_string());
//! }
//! 
//! // to avoid unintended behaviours always remember to close (.finish) after building the dawg
//! 
//! dawgie.finish();
//! 
//! 
//! assert!(dawgie.lookup("BATH").is_some());
//! assert!(dawgie.is_some());
//! ```

// mod repository;
mod dawg;
mod node;

pub use crate::dawg::dawg::Dawg;
pub use crate::node::node::Node;

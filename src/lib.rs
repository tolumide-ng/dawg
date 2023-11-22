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
//! features = [
//!     "threading" # Support Send + Sync 
//! ]
//! ```

// mod repository;
mod dawg;
mod node;

pub use crate::dawg::dawg::Dawg;
pub use crate::node::node::Node;

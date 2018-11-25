#[macro_use]
extern crate failure;

extern crate itertools;

extern crate lazy_init;

extern crate petgraph;

mod error;
pub use error::{GraphError, ReadError};

pub mod graph;

mod graph_algo;
pub(crate) use graph_algo::BfsWithDepth;

pub mod io;

pub mod proj;

pub mod token;

#[cfg(test)]
mod tests;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
#[macro_use]
extern crate maplit;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

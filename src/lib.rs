#[macro_use]
extern crate error_chain;

extern crate petgraph;

mod error;
pub use error::{Error, ErrorKind, Result};

mod features;
pub use features::Features;

mod graph;
pub(crate) use graph::BfsWithDepth;

mod reader;
pub use reader::{ReadSentence, Reader, Sentences};

mod token;
pub use token::{Sentence, Token, TokenBuilder};

mod writer;
pub use writer::{PartitioningWriter, WriteSentence, Writer};

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod graph_tests;

#[cfg(test)]
mod tests;

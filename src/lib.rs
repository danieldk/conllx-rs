#[macro_use]
extern crate failure;

extern crate itertools;

extern crate lazy_init;

extern crate petgraph;

mod error;
pub use error::{ReadError, GraphError};

mod features;
pub use features::Features;

mod graph;
pub(crate) use graph::BfsWithDepth;

mod proj;
pub use proj::{Deprojectivize, HeadProjectivizer, Projectivize};

#[cfg(test)]
pub(crate) use proj::{non_projective_edges, sentence_to_graph};

mod reader;
pub use reader::{ReadSentence, Reader, Sentences};

mod token;
pub use token::{Sentence, Token, TokenBuilder};

mod writer;
pub use writer::{PartitioningWriter, WriteSentence, Writer};

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

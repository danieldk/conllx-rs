#[macro_use]
extern crate error_chain;

mod error;
pub use error::{Error, ErrorKind, Result};

mod reader;
pub use reader::{ReadSentence, Reader, Sentences};

mod token;
pub use token::{Features, Sentence, Token, TokenBuilder};

mod writer;
pub use writer::{PartitioningWriter, WriteSentence, Writer};

#[cfg(test)]
mod tests;

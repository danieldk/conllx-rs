use std::io;

use error::Error;
use token::Sentence;

/// A trait for objects that can write CoNLL-X `Sentence`s.
pub trait WriteSentence {
    /// Write a `Sentence` into this object.
    ///
    /// # Errors
    ///
    /// A call to `write_sentence` may generate an error to indicate that
    /// the operation could not be completed.
    fn write_sentence(&mut self, sentence: &Sentence) -> Result<(), Error>;
}

/// A writer for CoNLL-X sentences.
///
/// This writer will write sentences to the embedded writer in CoNLL-X
/// tabular format.
pub struct Writer<W> {
    write: W,
    first: bool,
}

impl<W: io::Write> Writer<W> {
    /// Construct a new writer from an object that implements the `io::Write`
    /// trait.
    pub fn new(write: W) -> Writer<W> {
        Writer {
            write: write,
            first: true,
        }
    }

    /// Borrow the embedded writer. Getting the underlying writer is often
    /// useful when the writer writes to a memory object.
    ///
    /// # Examples
    ///
    /// ```
    /// use conllx::{Sentence, Token, WriteSentence, Writer};
    /// use std::str;
    ///
    /// let output = Vec::new();
    /// let mut writer = Writer::new(output);
    /// let sent = Sentence::new(vec![
    ///   Token::new(),
    ///   Token::new(),
    /// ]);
    ///
    /// writer.write_sentence(&sent).unwrap();
    ///
    /// println!("Output:\n{}", str::from_utf8(writer.get_ref()).unwrap());
    /// ```
    pub fn get_ref(&self) -> &W {
        &self.write
    }
}

impl<W: io::Write> WriteSentence for Writer<W> {
    fn write_sentence(&mut self, sentence: &Sentence) -> Result<(), Error> {
        if self.first {
            self.first = false;
            try!(write!(self.write, "{}", sentence))
        } else {
            try!(write!(self.write, "\n\n{}", sentence))
        }

        Ok(())
    }
}

/// A writer for CoNLL-X sentences that partitions incoming objects
/// among multiple writers.
///
/// For example, suppose that a `PartitioningWriter` is wraps writers
/// *w1*, *w2*, and sentences *s[1-5]* are written. The sentences are then
/// written as follows:
///
/// * s1 -> w1
/// * s2 -> w2
/// * s3 -> w1
/// * s4 -> w2
/// * s5 -> w1
pub struct PartitioningWriter<W>
    where W: WriteSentence
{
    writers: Vec<W>,
    fold: usize,
}

impl<W> PartitioningWriter<W>
    where W: WriteSentence
{
    pub fn new(writers: Vec<W>) -> PartitioningWriter<W> {
        PartitioningWriter {
            writers: writers,
            fold: 0,
        }
    }
}

impl<W> WriteSentence for PartitioningWriter<W>
    where W: WriteSentence
{
    fn write_sentence(&mut self, sentence: &Sentence) -> Result<(), Error> {
        if self.fold == self.writers.len() {
            self.fold = 0
        }

        try!(self.writers[self.fold].write_sentence(sentence));
        self.fold += 1;

        Ok(())
    }
}

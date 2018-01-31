use std::io;
use std::io::Result;

use token::{DisplaySentence, Token};

/// A trait for objects that can write CoNLL-X `Sentence`s.
pub trait WriteSentence {
    /// Write a sentence into this object.
    ///
    /// # Errors
    ///
    /// A call to `write_sentence` may generate an error to indicate that
    /// the operation could not be completed.
    fn write_sentence<S>(&mut self, sentence: S) -> Result<()> where S: AsRef<[Token]>;
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
    /// use conllx::{Token, WriteSentence, Writer};
    /// use std::str;
    ///
    /// let output = Vec::new();
    /// let mut writer = Writer::new(output);
    /// let sent = vec![
    ///   Token::new("hello"),
    ///   Token::new("world"),
    /// ];
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
    fn write_sentence<S>(&mut self, sentence: S) -> Result<()> where S: AsRef<[Token]> {
        if self.first {
            self.first = false;
            write!(self.write, "{}", DisplaySentence(sentence.as_ref()))?
        } else {
            write!(self.write, "\n\n{}", DisplaySentence(sentence.as_ref()))?
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
where
    W: WriteSentence,
{
    writers: Vec<W>,
    fold: usize,
}

impl<W> PartitioningWriter<W>
where
    W: WriteSentence,
{
    pub fn new(writers: Vec<W>) -> PartitioningWriter<W> {
        PartitioningWriter {
            writers: writers,
            fold: 0,
        }
    }
}

impl<W> WriteSentence for PartitioningWriter<W>
where
    W: WriteSentence,
{
    fn write_sentence<S>(&mut self, sentence: S) -> Result<()> where S: AsRef<[Token]> {
        if self.fold == self.writers.len() {
            self.fold = 0
        }

        self.writers[self.fold].write_sentence(sentence)?;
        self.fold += 1;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io;
    use std::io::Read;
    use std::str;

    use super::{WriteSentence, Writer};
    use tests::TEST_SENTENCES;

    static EMPTY: &str = "testdata/empty.conll";

    fn read_file(filename: &str) -> io::Result<String> {
        let mut f = File::open(filename)?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;
        Ok(contents)
    }

    #[test]
    fn writer() {
        let output = Vec::new();
        let mut writer = Writer::new(Box::new(output));

        for sentence in &*TEST_SENTENCES {
            writer.write_sentence(&sentence).unwrap();
        }

        assert_eq!(
            read_file(EMPTY).unwrap(),
            str::from_utf8(writer.get_ref()).unwrap()
        );
    }
}

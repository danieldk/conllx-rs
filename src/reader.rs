use std::io;

use token::{Sentence, Token, EMPTY_TOKEN};
use error::Error;

/// A trait for objects that can read CoNLL-X `Sentence`s
pub trait ReadSentence {
    /// Read a `Sentence` from this object.
    ///
    /// # Errors
    ///
    /// A call to `read_sentence` may generate an error to indicate that
    /// the operation could not be completed.
    fn read_sentence(&mut self) -> Result<Option<Sentence>, Error>;
}

/// Reader for CoNLL-X sentences.
pub struct Reader<R> {
    read: R,
}

impl<R: io::BufRead> Reader<R> {
    /// Construct a new reader from an object that implements the
    /// `io::BufRead` trait.
    pub fn new(read: R) -> Reader<R> {
        Reader { read: read }
    }

    /// Get an iterator over the `Sentence`s in this reader.
    pub fn sentences(self) -> Sentences<R> {
        Sentences { reader: self }
    }
}

impl<R: io::BufRead> IntoIterator for Reader<R> {
    type Item = Result<Sentence, Error>;
    type IntoIter = Sentences<R>;

    fn into_iter(self) -> Self::IntoIter {
        self.sentences()
    }
}

impl<R: io::BufRead> ReadSentence for Reader<R> {
    fn read_sentence(&mut self) -> Result<Option<Sentence>, Error> {
        let mut line = String::new();
        let mut tokens = Vec::new();

        loop {
            line.clear();

            // End of reader.
            if try!(self.read.read_line(&mut line)) == 0 {
                if tokens.is_empty() {
                    return Ok(None);
                }

                return Ok(Some(Sentence::new(tokens)));
            }

            // The blank line is a sentence separator. We want to be robust
            // in the case a CoNLL file is malformed and has two newlines as
            // a separator.
            if line.trim().is_empty() {
                if tokens.is_empty() {
                    continue;
                }

                return Ok(Some(Sentence::new(tokens)));
            }

            let mut iter = line.trim().split_terminator('\t');

            try!(parse_numeric_field(iter.next()));

            let mut token = Token::new();
            token.set_form(parse_string_field(iter.next()));
            token.set_lemma(parse_string_field(iter.next()));
            token.set_cpos(parse_string_field(iter.next()));
            token.set_pos(parse_string_field(iter.next()));
            token.set_features(parse_string_field(iter.next()));
            token.set_head(try!(parse_numeric_field(iter.next())));
            token.set_head_rel(parse_string_field(iter.next()));
            token.set_p_head(try!(parse_numeric_field(iter.next())));
            token.set_p_head_rel(parse_string_field(iter.next()));

            tokens.push(token);
        }
    }
}

/// Sentences is an iterator over the sentences in a `Reader`.
pub struct Sentences<R> {
    reader: Reader<R>,
}

impl<R: io::BufRead> Iterator for Sentences<R> {
    type Item = Result<Sentence, Error>;

    fn next(&mut self) -> Option<Result<Sentence, Error>> {
        match self.reader.read_sentence() {
            Ok(None) => None,
            Ok(Some(sent)) => Some(Ok(sent)),
            Err(e) => Some(Err(e)),
        }
    }
}

fn parse_string_field(field: Option<&str>) -> Option<String> {
    field.and_then(|s| {
        if s == EMPTY_TOKEN {
            None
        } else {
            Some(s.to_string())
        }
    })
}

fn parse_numeric_field(field: Option<&str>) -> Result<Option<usize>, Error> {
    match field {
        None => Ok(None),
        Some(s) => {
            if s == EMPTY_TOKEN {
                return Ok(None);
            } else {
                Ok(Some(try!(s.parse())))
            }
        }
    }
}

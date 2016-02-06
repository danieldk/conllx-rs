use std::fmt;
use std::io;
use std::num;

use token::{Sentence, Token, EMPTY_TOKEN};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Parse(num::ParseIntError),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Error {
        Error::Parse(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => write!(f, "{}", err),
            Error::Parse(ref err) => write!(f, "{}", err),
        }
    }
}

pub trait ReadSentence {
    fn read_sentence(&mut self) -> Result<Option<Sentence>, Error>;
}

pub struct Reader<R> {
    read: R,
}

impl<R: io::BufRead> Reader<R> {
    pub fn new(read: R) -> Reader<R> {
        Reader { read: read }
    }

    pub fn sentences(self) -> Sentences<R> {
        Sentences { reader: self }
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

            let tok = Token::new_from(parse_string_field(iter.next()),
                                      parse_string_field(iter.next()),
                                      parse_string_field(iter.next()),
                                      parse_string_field(iter.next()),
                                      parse_string_field(iter.next()),
                                      try!(parse_numeric_field(iter.next())),
                                      parse_string_field(iter.next()),
                                      try!(parse_numeric_field(iter.next())),
                                      parse_string_field(iter.next()));

            tokens.push(tok);
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

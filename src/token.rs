use std::fmt;
use std::ops;
use std::slice;
use std::vec;

use features::Features;

/// A sentence with the CoNLL-X annotation layers.
///
/// This data type is a small wrapper around `Vec<Token>` that provides some
/// extra convenience. For example, the implementation of the `Display` trait
/// outputs the sentence in CoNLL-X format.
#[derive(Clone, Debug,PartialEq)]
pub struct Sentence {
    tokens: Vec<Token>,
}

impl Sentence {
    /// Create a new `Sentence` from a vector of tokens.
    pub fn new(tokens: Vec<Token>) -> Sentence {
        Sentence { tokens: tokens }
    }

    /// Get the underlying vector of tokens.
    pub fn as_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    /// Get an iterator over the sentence tokens.
    pub fn iter(&self) -> slice::Iter<Token> {
        self.tokens.iter()
    }

    /// Get a mutable iterator over the sentence tokens.
    pub fn iter_mut(&mut self) -> slice::IterMut<Token> {
        self.tokens.iter_mut()
    }
}

impl fmt::Display for Sentence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let last_id = self.tokens.len() - 1;

        for (id, token) in self.iter().enumerate() {
            if id == last_id {
                write!(f, "{}\t{}", id + 1, token)?
            } else {
                write!(f, "{}\t{}\n", id + 1, token)?
            }
        }

        Ok(())
    }
}

impl ops::Index<usize> for Sentence {
    type Output = Token;
    fn index(&self, index: usize) -> &Token {
        &self.tokens[index]
    }
}

impl ops::IndexMut<usize> for Sentence {
    fn index_mut(&mut self, index: usize) -> &mut Token {
        &mut self.tokens[index]
    }
}

impl IntoIterator for Sentence {
    type Item = Token;
    type IntoIter = vec::IntoIter<Token>;

    fn into_iter(self) -> vec::IntoIter<Token> {
        self.tokens.into_iter()
    }
}

impl<'a> IntoIterator for &'a Sentence {
    type Item = &'a Token;
    type IntoIter = slice::Iter<'a, Token>;

    fn into_iter(self) -> slice::Iter<'a, Token> {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Sentence {
    type Item = &'a mut Token;
    type IntoIter = slice::IterMut<'a, Token>;

    fn into_iter(mut self) -> slice::IterMut<'a, Token> {
        self.iter_mut()
    }
}

/// A builder for `Token`s.
///
/// The `Token` type stores a CoNLL-X token. However, since this format
/// permits a large number of fields, construction of a token can get
/// tedious. This builder provides a fluent interface for creating `Token`s.
pub struct TokenBuilder {
    token: Token,
}

impl TokenBuilder {
    /// Create a `Token` builder with all fields set to absent.
    pub fn new() -> TokenBuilder {
        TokenBuilder { token: Token::new() }
    }

    /// Set the word form or punctuation symbol.
    pub fn form<S>(mut self, form: S) -> TokenBuilder
        where S: Into<String>
    {
        self.token.set_form(Some(form));
        self
    }

    /// Set the lemma or stem of the word form.
    pub fn lemma<S>(mut self, lemma: S) -> TokenBuilder
        where S: Into<String>
    {
        self.token.set_lemma(Some(lemma));
        self
    }

    /// Set the coarse-grained part-of-speech tag.
    pub fn cpos<S>(mut self, cpos: S) -> TokenBuilder
        where S: Into<String>
    {
        self.token.set_cpos(Some(cpos));
        self
    }

    /// Set the fine-grained part-of-speech tag.
    pub fn pos<S>(mut self, pos: S) -> TokenBuilder
        where S: Into<String>
    {
        self.token.set_pos(Some(pos));
        self
    }

    /// Set the syntactic and/or morphological features of the token.
    pub fn features(mut self, features: Features) -> TokenBuilder {
        self.token.set_features(Some(features));
        self
    }

    /// Set the head of the token. This is the sentence position
    /// of the head **plus one**. If the head is 0, the token the root
    /// of the dependency tree.
    pub fn head(mut self, head: usize) -> TokenBuilder {
        self.token.set_head(Some(head));
        self
    }

    /// Set the dependency relation to the head of this token.
    pub fn head_rel<S>(mut self, head_rel: S) -> TokenBuilder
        where S: Into<String>
    {
        self.token.set_head_rel(Some(head_rel));
        self
    }

    /// Set the projective head of the token. This is the sentence position
    /// of the head **plus one**. If the head is 0, the token the root
    /// of the dependency tree. The dependency structure resulting from the
    /// projective heads must be projective.
    pub fn p_head(mut self, p_head: usize) -> TokenBuilder {
        self.token.set_p_head(Some(p_head));
        self
    }

    /// Set the dependency relation to the projective head of this token.
    pub fn p_head_rel<S>(mut self, p_head_rel: S) -> TokenBuilder
        where S: Into<String>
    {
        self.token.set_p_head_rel(Some(p_head_rel));
        self
    }

    pub fn token(self) -> Token {
        self.token
    }
}

/// A token with the CoNLL-X annotation layers.
///
/// The fields of CoNLLX tokens are described at:
///
/// http://ilk.uvt.nl/conll/
///
/// This type provides exactly the same fields, except for the ID field
/// (since it can be derived from the sentence position of the token).
/// If a particular field is absent (*_* in the CoNLL-X format), its
/// value is `None`.
#[derive(Clone,Debug,PartialEq)]
pub struct Token {
    form: Option<String>,
    lemma: Option<String>,
    cpos: Option<String>,
    pos: Option<String>,
    features: Option<Features>,
    head: Option<usize>,
    head_rel: Option<String>,
    p_head: Option<usize>,
    p_head_rel: Option<String>,
}

impl Token {
    /// Create a new token where all the fields are absent.
    pub fn new() -> Token {
        Token {
            form: None,
            lemma: None,
            cpos: None,
            pos: None,
            features: None,
            head: None,
            head_rel: None,
            p_head: None,
            p_head_rel: None,
        }
    }

    /// Get the word form or punctuation symbol.
    pub fn form(&self) -> Option<&str> {
        self.form.as_ref().map(String::as_ref)
    }

    /// Get the lemma or stem of the word form.
    pub fn lemma(&self) -> Option<&str> {
        self.lemma.as_ref().map(String::as_ref)
    }

    /// Get the coarse-grained part-of-speech tag.
    pub fn cpos(&self) -> Option<&str> {
        self.cpos.as_ref().map(String::as_ref)
    }

    /// Get the fine-grained part-of-speech tag.
    pub fn pos(&self) -> Option<&str> {
        self.pos.as_ref().map(String::as_ref)
    }

    /// Get the syntactic and/or morphological features of the token.
    pub fn features(&self) -> Option<&Features> {
        self.features.as_ref()
    }

    /// Get the head of the token. This is the sentence position
    /// of the head **plus one**. If the head is 0, the token the root
    /// of the dependency tree.
    pub fn head(&self) -> Option<usize> {
        self.head
    }

    /// Get the dependency relation to the head of this token.
    pub fn head_rel(&self) -> Option<&str> {
        self.head_rel.as_ref().map(String::as_ref)
    }

    /// Get the projective head of the token. This is the sentence position
    /// of the head **plus one**. If the head is 0, the token the root
    /// of the dependency tree. The dependency structure resulting from the
    /// projective heads must be projective.
    pub fn p_head(&self) -> Option<usize> {
        self.p_head
    }

    /// Get the dependency relation to the projective head of this token.
    pub fn p_head_rel(&self) -> Option<&str> {
        self.p_head_rel.as_ref().map(String::as_ref)
    }

    /// Set the word form or punctuation symbol.
    pub fn set_form<S>(&mut self, form: Option<S>)
        where S: Into<String>
    {
        self.form = form.map(|i| i.into())
    }

    /// Set the lemma or stem of the word form.
    pub fn set_lemma<S>(&mut self, lemma: Option<S>)
        where S: Into<String>
    {
        self.lemma = lemma.map(|i| i.into())
    }

    /// Set the coarse-grained part-of-speech tag.
    pub fn set_cpos<S>(&mut self, cpos: Option<S>)
        where S: Into<String>
    {
        self.cpos = cpos.map(|i| i.into())
    }

    /// Set the fine-grained part-of-speech tag.
    pub fn set_pos<S>(&mut self, pos: Option<S>)
        where S: Into<String>
    {
        self.pos = pos.map(|i| i.into())
    }

    /// Set the syntactic and/or morphological features of the token.
    pub fn set_features(&mut self, features: Option<Features>) {
        self.features = features
    }

    /// Set the head of the token. This is the sentence position
    /// of the head **plus one**. If the head is 0, the token the root
    /// of the dependency tree.
    pub fn set_head(&mut self, head: Option<usize>) {
        self.head = head
    }

    /// Set the dependency relation to the head of this token.
    pub fn set_head_rel<S>(&mut self, head_rel: Option<S>)
        where S: Into<String>
    {
        self.head_rel = head_rel.map(|i| i.into())
    }

    /// Set the projective head of the token. This is the sentence position
    /// of the head **plus one**. If the head is 0, the token the root
    /// of the dependency tree. The dependency structure resulting from the
    /// projective heads must be projective.
    pub fn set_p_head(&mut self, p_head: Option<usize>) {
        self.p_head = p_head
    }

    /// Set the dependency relation to the projective head of this token.
    pub fn set_p_head_rel<S>(&mut self, p_head_rel: Option<S>)
        where S: Into<String>
    {
        self.p_head_rel = p_head_rel.map(|i| i.into())
    }
}

pub const EMPTY_TOKEN: &'static str = "_";

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let head_str = self.head.as_ref().map(|n| n.to_string());
        let p_head_str = self.p_head.as_ref().map(|n| n.to_string());

        write!(f,
               "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
               self.form.as_ref().map(|s| s.as_ref()).unwrap_or(EMPTY_TOKEN),
               self.lemma.as_ref().map(|s| s.as_ref()).unwrap_or(EMPTY_TOKEN),
               self.cpos.as_ref().map(|s| s.as_ref()).unwrap_or(EMPTY_TOKEN),
               self.pos.as_ref().map(|s| s.as_ref()).unwrap_or(EMPTY_TOKEN),
               self.features.as_ref().map(|s| s.as_str()).unwrap_or(EMPTY_TOKEN),
               head_str.as_ref().map(|s| s.as_ref()).unwrap_or(EMPTY_TOKEN),
               self.head_rel.as_ref().map(|s| s.as_ref()).unwrap_or(EMPTY_TOKEN),
               self.p_head.clone().map(|n| n.to_string()).unwrap_or(EMPTY_TOKEN.to_string()),
               p_head_str.as_ref().map(|s| s.as_ref()).unwrap_or(EMPTY_TOKEN))
    }
}

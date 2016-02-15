use std::collections;
use std::fmt;
use std::ops;
use std::slice;

/// A sentence consists of zero or more `Token`s. It is a small wrapper
/// around `Vec<Token>` that provides some extra convenience. For example,
/// the implementation of the `Display` trait outputs the sentence in
/// CoNLL-X format.
#[derive(Debug,PartialEq)]
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
                try!(write!(f, "{}\t{}", id + 1, token))
            } else {
                try!(write!(f, "{}\t{}\n", id + 1, token))
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
    pub fn features<S>(mut self, features: S) -> TokenBuilder
        where S: Into<String>
    {
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

/// A token in the CoNLL-X dependency format.
///
/// The fields of CoNLLX tokens are described at:
///
/// http://ilk.uvt.nl/conll/
///
/// This type provides exactly the same fields, except for the ID field
/// (since it can be derived from the sentence position of the token).
/// If a particular field is absent (*_* in the CoNLL-X format), its
/// value is `None`.
#[derive(Debug,PartialEq)]
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

    /// Create a new token from the specified fields.
    pub fn from_fields<S>(form: Option<S>,
                          lemma: Option<S>,
                          cpos: Option<S>,
                          pos: Option<S>,
                          features: Option<S>,
                          head: Option<usize>,
                          head_rel: Option<S>,
                          p_head: Option<usize>,
                          p_head_rel: Option<S>)
                          -> Token
        where S: Into<String>
    {
        Token {
            form: form.map(|i| i.into()),
            lemma: lemma.map(|i| i.into()),
            cpos: cpos.map(|i| i.into()),
            pos: pos.map(|i| i.into()),
            features: features.map(|s| Features { features: s.into() }),
            head: head,
            head_rel: head_rel.map(|i| i.into()),
            p_head: p_head,
            p_head_rel: p_head_rel.map(|i| i.into()),
        }
    }

    /// Get the word form or punctuation symbol.
    pub fn form(&self) -> &Option<String> {
        &self.form
    }

    /// Get the lemma or stem of the word form.
    pub fn lemma(&self) -> &Option<String> {
        &self.lemma
    }

    /// Get the coarse-grained part-of-speech tag.
    pub fn cpos(&self) -> &Option<String> {
        &self.cpos
    }

    /// Get the fine-grained part-of-speech tag.
    pub fn pos(&self) -> &Option<String> {
        &self.pos
    }

    /// Get the syntactic and/or morphological features of the token.
    pub fn features(&self) -> &Option<Features> {
        &self.features
    }

    /// Get the head of the token. This is the sentence position
    /// of the head **plus one**. If the head is 0, the token the root
    /// of the dependency tree.
    pub fn head(&self) -> &Option<usize> {
        &self.head
    }

    /// Get the dependency relation to the head of this token.
    pub fn head_rel(&self) -> &Option<String> {
        &self.head_rel
    }

    /// Get the projective head of the token. This is the sentence position
    /// of the head **plus one**. If the head is 0, the token the root
    /// of the dependency tree. The dependency structure resulting from the
    /// projective heads must be projective.
    pub fn p_head(&self) -> &Option<usize> {
        &self.p_head
    }

    /// Get the dependency relation to the projective head of this token.
    pub fn p_head_rel(&self) -> &Option<String> {
        &self.p_head_rel
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
    pub fn set_features<S>(&mut self, features: Option<S>)
        where S: Into<String>
    {
        self.features = features.map(|s| Features { features: s.into() })
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

/// This data type is used to represent features on a `Token`. In the CoNLL-X
/// specification, these are morphological features of the token. However,
/// they are used as a free form list or mapping of features.
#[derive(Debug,PartialEq)]
pub struct Features {
    features: String,
}

impl Features {
    /// Get the features field as a key-value mapping. This assumes that
    /// the key-value pairs are separed using a vertical bar (`|`) and keys
    /// and values using a colon (`:`). If the value is absent, corresponding
    /// value in the mapping is `None`.
    pub fn as_map(&self) -> collections::BTreeMap<String, Option<String>> {
        let mut features = collections::BTreeMap::new();

        for fv in self.features.split('|') {
            let mut iter = fv.split(':');
            if let Some(k) = iter.next() {
                let v = iter.next().map(|s| s.to_owned());
                features.insert(k.to_owned(), v.to_owned());
            }
        }

        features
    }

    /// Get the features field.
    pub fn as_str(&self) -> &str {
        self.features.as_ref()
    }
}

impl fmt::Display for Features {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.features.as_ref())
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

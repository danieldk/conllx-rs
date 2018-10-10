use std::mem;

use features::Features;

pub const EMPTY_TOKEN: &'static str = "_";

/// A builder for `Token`s.
///
/// The `Token` type stores a CoNLL-X token. However, since this format
/// permits a large number of fields, construction of a token can get
/// tedious. This builder provides a fluent interface for creating `Token`s.
pub struct TokenBuilder {
    token: Token,
}

impl TokenBuilder {
    /// Create a `Token` builder with all non-form fields set to absent.
    pub fn new<S>(form: S) -> TokenBuilder
    where
        S: Into<String>,
    {
        TokenBuilder {
            token: Token::new(form),
        }
    }

    /// Set the word form or punctuation symbol.
    pub fn form<S>(mut self, form: S) -> TokenBuilder
    where
        S: Into<String>,
    {
        self.token.set_form(form);
        self
    }

    /// Set the lemma or stem of the word form.
    pub fn lemma<S>(mut self, lemma: S) -> TokenBuilder
    where
        S: Into<String>,
    {
        self.token.set_lemma(Some(lemma));
        self
    }

    /// Set the coarse-grained part-of-speech tag.
    pub fn cpos<S>(mut self, cpos: S) -> TokenBuilder
    where
        S: Into<String>,
    {
        self.token.set_cpos(Some(cpos));
        self
    }

    /// Set the fine-grained part-of-speech tag.
    pub fn pos<S>(mut self, pos: S) -> TokenBuilder
    where
        S: Into<String>,
    {
        self.token.set_pos(Some(pos));
        self
    }

    /// Set the syntactic and/or morphological features of the token.
    pub fn features(mut self, features: Features) -> TokenBuilder {
        self.token.set_features(Some(features));
        self
    }
}

impl From<Token> for TokenBuilder {
    fn from(token: Token) -> Self {
        TokenBuilder { token }
    }
}

impl From<TokenBuilder> for Token {
    fn from(builder: TokenBuilder) -> Self {
        builder.token
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Token {
    form: String,
    lemma: Option<String>,
    cpos: Option<String>,
    pos: Option<String>,
    features: Option<Features>,
}

impl Token {
    /// Create a new token where all the non-form fields are absent.
    pub fn new<S>(form: S) -> Token
    where
        S: Into<String>,
    {
        Token {
            form: form.into(),
            lemma: None,
            cpos: None,
            pos: None,
            features: None,
        }
    }

    /// Get the word form or punctuation symbol.
    pub fn form(&self) -> &str {
        self.form.as_ref()
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

    /// Set the word form or punctuation symbol.
    ///
    /// Returns the form that is replaced.
    pub fn set_form<S>(&mut self, form: S) -> String
    where
        S: Into<String>,
    {
        mem::replace(&mut self.form, form.into())
    }

    /// Set the lemma or stem of the word form.
    ///
    /// Returns the lemma that is replaced.
    pub fn set_lemma<S>(&mut self, lemma: Option<S>) -> Option<String>
    where
        S: Into<String>,
    {
        mem::replace(&mut self.lemma, lemma.map(|i| i.into()))
    }

    /// Set the coarse-grained part-of-speech tag.
    ///
    /// Returns the coarse-grained part-of-speech tag that is replaced.
    pub fn set_cpos<S>(&mut self, cpos: Option<S>) -> Option<String>
    where
        S: Into<String>,
    {
        mem::replace(&mut self.cpos, cpos.map(|i| i.into()))
    }

    /// Set the fine-grained part-of-speech tag.
    ///
    /// Returns the fine-grained part-of-speech tag that is replaced.
    pub fn set_pos<S>(&mut self, pos: Option<S>) -> Option<String>
    where
        S: Into<String>,
    {
        mem::replace(&mut self.pos, pos.map(|i| i.into()))
    }

    /// Set the syntactic and/or morphological features of the token.
    ///
    /// Returns the features that are replaced.
    pub fn set_features(&mut self, features: Option<Features>) -> Option<Features> {
        mem::replace(&mut self.features, features)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::{Features, Token, TokenBuilder};

    #[test]
    fn features() {
        let tokens = token_with_features();
        let features = features_correct();

        for (token, correct) in tokens.iter().zip(features) {
            let kv = token.features().as_ref().unwrap().as_map();
            assert_eq!(&correct, kv);
        }
    }

    fn token_with_features() -> Vec<Token> {
        vec![
            TokenBuilder::new("Gilles")
                .lemma("Gilles")
                .cpos("N")
                .pos("NE")
                .features(Features::from_string(
                    "case:nominative|number:singular|gender:masculine",
                )).into(),
            TokenBuilder::new("Deleuze")
                .lemma("Deleuze")
                .cpos("N")
                .pos("NE")
                .features(Features::from_string("nominative|singular|masculine"))
                .into(),
        ]
    }

    fn features_correct() -> Vec<BTreeMap<String, Option<String>>> {
        let mut correct1 = BTreeMap::new();
        correct1.insert("case".to_owned(), Some("nominative".to_owned()));
        correct1.insert("number".to_owned(), Some("singular".to_owned()));
        correct1.insert("gender".to_owned(), Some("masculine".to_owned()));

        let mut correct2 = BTreeMap::new();
        correct2.insert("nominative".to_owned(), None);
        correct2.insert("singular".to_owned(), None);
        correct2.insert("masculine".to_owned(), None);

        vec![correct1, correct2]
    }
}

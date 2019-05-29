//! Tokens in the dependency graph.

use std::collections::BTreeMap;
use std::fmt;
use std::fmt::{Debug, Display};
use std::iter::FromIterator;
use std::mem;

use itertools::Itertools;
use lazy_init::Lazy;

pub const EMPTY_TOKEN: &str = "_";

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
    pub fn new(form: impl Into<String>) -> TokenBuilder {
        TokenBuilder {
            token: Token::new(form),
        }
    }

    /// Set the word form or punctuation symbol.
    pub fn form(mut self, form: impl Into<String>) -> TokenBuilder {
        self.token.set_form(form);
        self
    }

    /// Set the lemma or stem of the word form.
    pub fn lemma(mut self, lemma: impl Into<String>) -> TokenBuilder {
        self.token.set_lemma(Some(lemma));
        self
    }

    /// Set the coarse-grained part-of-speech tag.
    pub fn cpos(mut self, cpos: impl Into<String>) -> TokenBuilder {
        self.token.set_cpos(Some(cpos));
        self
    }

    /// Set the fine-grained part-of-speech tag.
    pub fn pos(mut self, pos: impl Into<String>) -> TokenBuilder {
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
    pub fn new(form: impl Into<String>) -> Token {
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
    pub fn set_form(&mut self, form: impl Into<String>) -> String {
        mem::replace(&mut self.form, form.into())
    }

    /// Set the lemma or stem of the word form.
    ///
    /// Returns the lemma that is replaced.
    pub fn set_lemma<S>(&mut self, lemma: Option<S>) -> Option<String>
    where
        S: Into<String>,
    {
        mem::replace(&mut self.lemma, lemma.map(Into::into))
    }

    /// Set the coarse-grained part-of-speech tag.
    ///
    /// Returns the coarse-grained part-of-speech tag that is replaced.
    pub fn set_cpos<S>(&mut self, cpos: Option<S>) -> Option<String>
    where
        S: Into<String>,
    {
        mem::replace(&mut self.cpos, cpos.map(Into::into))
    }

    /// Set the fine-grained part-of-speech tag.
    ///
    /// Returns the fine-grained part-of-speech tag that is replaced.
    pub fn set_pos<S>(&mut self, pos: Option<S>) -> Option<String>
    where
        S: Into<String>,
    {
        mem::replace(&mut self.pos, pos.map(Into::into))
    }

    /// Set the syntactic and/or morphological features of the token.
    ///
    /// Returns the features that are replaced.
    pub fn set_features(&mut self, features: Option<Features>) -> Option<Features> {
        mem::replace(&mut self.features, features)
    }
}

/// Token features.
///
/// In the CoNLL-X specification, these are morphological features of the
/// token. Typically, the features are a list or a key-value mapping.
pub struct Features {
    features: String,
    feature_map: Lazy<BTreeMap<String, Option<String>>>,
}

impl Features {
    /// Create features from a string. The casual format uses key-value
    /// pairs that are separated by a vertical bar (`|`) and keys and
    /// values using a colon (`:`). Arbitrary strings will also be accepted,
    /// however they will not give a nice feature-value mapping when using
    /// `as_map`.
    pub fn from_string(s: impl Into<String>) -> Self {
        Features {
            features: s.into(),
            feature_map: Lazy::new(),
        }
    }

    /// Get the features field as a key-value mapping. This assumes that
    /// the key-value pairs are separed using a vertical bar (`|`) and keys
    /// and values using a colon (`:`). If the value is absent, corresponding
    /// value in the mapping is `None`.
    ///
    /// The feature map is constructed lazily:
    ///
    /// * If `as_map` is never called, the feature map is never created.
    /// * If `as_map` is called once or more, the feature map is only created
    ///   once.
    pub fn as_map(&self) -> &BTreeMap<String, Option<String>> {
        self.feature_map.get_or_create(|| self.as_map_eager())
    }

    /// Get the features field.
    pub fn as_str(&self) -> &str {
        self.features.as_ref()
    }

    /// Unwrap the contained feature string and map. Since the feature map is
    /// initialized lazily, this will force initialization of the feature map
    /// when necessary.
    pub fn into_inner(self) -> (String, BTreeMap<String, Option<String>>) {
        let _ = self.feature_map.get_or_create(|| self.as_map_eager());
        (
            self.features,
            self.feature_map
                .into_inner()
                .expect("feature map should have been initialized"),
        )
    }

    /// Unwrap the contained feature map. Since the feature map is initialized
    /// lazily, this will force initialization of the feature map when necessary.
    pub fn into_inner_map(self) -> BTreeMap<String, Option<String>> {
        let _ = self.feature_map.get_or_create(|| self.as_map_eager());
        self.feature_map
            .into_inner()
            .expect("feature map should have been initialized")
    }

    /// Unwrap the contained feature string.
    pub fn into_inner_string(self) -> String {
        self.features
    }

    fn as_map_eager(&self) -> BTreeMap<String, Option<String>> {
        let mut features = BTreeMap::new();

        for fv in self.features.split('|') {
            let fv: &str = fv;
            let (k, v) = fv
                .find(':')
                .map(|idx| (fv[..idx].to_owned(), Some(fv[idx + 1..].to_owned())))
                .unwrap_or_else(|| (fv.to_owned(), None));
            features.insert(k, v);
        }

        features
    }
}

impl Clone for Features {
    fn clone(&self) -> Self {
        Features {
            features: self.features.clone(),
            feature_map: Lazy::new(),
        }
    }
}

impl Debug for Features {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Features {{ features: {} }}", self.features)
    }
}

impl Display for Features {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.features.as_ref())
    }
}

impl Eq for Features {}

impl<S, T> FromIterator<(S, Option<T>)> for Features
where
    S: Into<String>,
    T: Into<String>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (S, Option<T>)>,
    {
        let feature_map =
            BTreeMap::from_iter(iter.into_iter().map(|(k, v)| (k.into(), v.map(Into::into))));
        let features = map_to_string(&feature_map);

        let lazy_feature_map = Lazy::new();
        lazy_feature_map.get_or_create(|| feature_map);

        Features {
            features,
            feature_map: lazy_feature_map,
        }
    }
}

impl PartialEq for Features {
    fn eq(&self, other: &Features) -> bool {
        self.feature_map.get_or_create(|| self.as_map_eager())
            == other.feature_map.get_or_create(|| other.as_map_eager())
    }
}

fn map_to_string(feature_map: &BTreeMap<String, Option<String>>) -> String {
    feature_map
        .iter()
        .map(|(k, v)| match *v {
            Some(ref v) => format!("{}:{}", k, v),
            None => k.to_owned(),
        })
        .join("|")
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::iter::FromIterator;

    use maplit::btreemap;
    use quickcheck::quickcheck;

    use super::{Features, Token, TokenBuilder};

    quickcheck! {
        fn features_from_iter(feature_map: BTreeMap<String, Option<String>>) -> bool{
            &feature_map == Features::from_iter(feature_map.clone()).as_map()
        }
    }

    #[test]
    fn features_from_iter_as_string() {
        let feature_map = btreemap! {
            "feature2" => Some("y"),
            "feature3" => None,
            "feature1" => Some("x")
        };

        let features = Features::from_iter(feature_map);

        assert_eq!(features.as_str(), "feature1:x|feature2:y|feature3");
    }

    #[test]
    fn features_with_colons() {
        let f = "Some:feature:with|additional:colons|feature";
        let features = Features::from_string(f);
        let some = features
            .as_map()
            .get("Some")
            .unwrap()
            .as_ref()
            .map(String::as_str);
        assert_eq!(some, Some("feature:with"));
        let additional = features
            .as_map()
            .get("additional")
            .unwrap()
            .as_ref()
            .map(String::as_str);
        assert_eq!(additional, Some("colons"));
        let feature = features
            .as_map()
            .get("feature")
            .unwrap()
            .as_ref()
            .map(String::as_str);
        assert_eq!(feature, None);
    }

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
                ))
                .into(),
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

    #[test]
    fn eq_features_is_order_insensitive() {
        let token1: Token = TokenBuilder::new("a")
            .features(Features::from_string("a|b:c"))
            .into();
        let token2 = TokenBuilder::new("a")
            .features(Features::from_string("b:c|a"))
            .into();
        let token3: Token = TokenBuilder::new("a")
            .features(Features::from_string("b|a:c"))
            .into();

        assert_eq!(token1, token2);
        assert_ne!(token1, token3);
        assert_ne!(token2, token3);
    }
}

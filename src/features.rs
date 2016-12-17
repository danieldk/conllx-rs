use std::collections::BTreeMap;
use std::fmt;

/// Token features.
///
/// In the CoNLL-X specification, these are morphological features of the
/// token. Typically, the features are a list or a key-value mapping.
#[derive(Clone,Debug,PartialEq)]
pub struct Features {
    features: String,
}

impl Features {
    /// Create features from a string. The casual format uses key-value
    /// pairs that are separated by a vertical bar (`|`) and keys and
    /// values using a colon (`:`). Arbitrary strings will also be expected,
    /// however they will not give a nice feature-value mapping when using
    /// `as_map`.
    pub fn from_string<S>(s: S) -> Self
        where S: Into<String>
    {
        Features { features: s.into() }
    }

    /// Get the features field as a key-value mapping. This assumes that
    /// the key-value pairs are separed using a vertical bar (`|`) and keys
    /// and values using a colon (`:`). If the value is absent, corresponding
    /// value in the mapping is `None`.
    pub fn as_map(&self) -> BTreeMap<String, Option<String>> {
        let mut features = BTreeMap::new();

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

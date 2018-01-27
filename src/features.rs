use std::collections::BTreeMap;
use std::fmt;
use std::fmt::{Debug, Display};
use std::iter::FromIterator;

use itertools::Itertools;
use lazy_init::Lazy;

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
    pub fn from_string<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Features {
            features: s.into(),
            feature_map: Lazy::new(),
        }
    }

    pub fn from_iter<I, S, T>(iter: I) -> Self
    where
        I: IntoIterator<Item = (S, Option<T>)>,
        S: Into<String>,
        T: Into<String>,
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

    fn as_map_eager(&self) -> BTreeMap<String, Option<String>> {
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

impl PartialEq for Features {
    fn eq(&self, other: &Features) -> bool {
        self.features.eq(&other.features)
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

    use super::Features;

    #[test]
    quickcheck! {
        fn from_iter(feature_map: BTreeMap<String, Option<String>>) -> bool{
            &feature_map == Features::from_iter(feature_map.clone()).as_map()
        }
    }

    #[test]
    fn from_iter_as_string() {
        let feature_map = btreemap!{
            "feature2" => Some("y"),
            "feature3" => None,
            "feature1" => Some("x")
        };

        let features = Features::from_iter(feature_map);

        assert_eq!(features.as_str(), "feature1:x|feature2:y|feature3");
    }
}

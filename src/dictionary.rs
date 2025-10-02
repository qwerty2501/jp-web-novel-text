use std::fmt::Display;

use derive_getters::Getters;
use derive_new::new;

#[derive(Getters, new, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DictionaryWordRubySpecific {
    ruby: String,
    char_index: usize,
}

#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DictionaryWordRuby {
    All(String),
    Specifics(Vec<DictionaryWordRubySpecific>),
}

impl DictionaryWordRuby {
    pub fn is_empty(&self) -> bool {
        match self {
            Self::All(s) => s.is_empty(),
            Self::Specifics(spec_rubys) => spec_rubys.is_empty(),
        }
    }
}

impl Display for DictionaryWordRuby {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::All(s) => f.write_str(s),
            Self::Specifics(spec_rubys) => {
                for s in spec_rubys.iter().map(|s| s.ruby().as_str()) {
                    f.write_str(s)?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Getters, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DictionaryWord<X = ()> {
    key: String,
    ruby: DictionaryWordRuby,
    description: String,
    extra: X,
}

impl DictionaryWord {
    pub fn new(key: String, ruby: String, description: String) -> Self {
        Self::new_extra(key, DictionaryWordRuby::All(ruby), description, ())
    }
}

impl<X> DictionaryWord<X> {
    pub fn new_extra(key: String, ruby: DictionaryWordRuby, description: String, extra: X) -> Self {
        Self {
            key,
            ruby,
            description,
            extra,
        }
    }
}

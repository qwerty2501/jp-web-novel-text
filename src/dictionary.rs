use derive_getters::Getters;

use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::{Result, parse_dictionary::DoubleArrayDictionary};

#[derive(Clone, new, PartialEq, Debug, Serialize, Deserialize)]
pub enum DictionaryWordKeyPhrase {
    Plain { target: String },
    Ruby { target: String, ruby: String },
}

#[derive(Getters, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct DictionaryWord<X = ()> {
    key: String,
    phrase: Vec<DictionaryWordKeyPhrase>,
    description: String,
    extra: X,
}

impl DictionaryWord {
    pub fn new(key: String, ruby: String, description: String) -> Self {
        Self::new_all(
            vec![if !ruby.is_empty() {
                DictionaryWordKeyPhrase::new_ruby(key, ruby)
            } else {
                DictionaryWordKeyPhrase::new_plain(key)
            }],
            description,
            (),
        )
    }
}

impl<X> DictionaryWord<X> {
    pub fn new_all(phrase: Vec<DictionaryWordKeyPhrase>, description: String, extra: X) -> Self {
        let mut key = String::new();
        for rp in phrase.iter() {
            match rp {
                DictionaryWordKeyPhrase::Plain { target } => key.push_str(target),
                DictionaryWordKeyPhrase::Ruby { target, ruby: _ } => key.push_str(target),
            }
        }
        Self {
            key,
            phrase,
            description,
            extra,
        }
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct PreparedDictionary<X: Clone = ()> {
    format_version: String,
    words: DictionaryWord<X>,
    trie_vec: Vec<u8>,
}

impl<X: Clone> PreparedDictionary<X> {
    pub(crate) fn format_version(&self) -> &str {
        &self.format_version
    }

    pub fn prepare(words: Vec<DictionaryWord<X>>) -> Result<Self> {
        let da_dic = DoubleArrayDictionary::try_new(words.clone())?;
    }
}

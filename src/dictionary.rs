use derive_getters::Getters;

use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::{
    Error, Result, general_parser::DictionaryWordContainer, parse_dictionary::DoubleArrayDictionary,
};

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
pub struct PreparedDictionary<WD>
where
    WD: Clone + DictionaryWordContainer,
{
    format_version: String,
    pub(crate) words: Vec<WD>,
    pub(crate) trie_vec: Vec<u8>,
}

impl<WD> PreparedDictionary<WD>
where
    WD: Clone + DictionaryWordContainer,
{
    pub(crate) const CURRENT_FORMAT_VERSION: &str = "1.0.0";
    pub(crate) fn format_version(&self) -> &str {
        &self.format_version
    }

    pub fn prepare(words: Vec<WD>) -> Result<Self> {
        let da_dic = DoubleArrayDictionary::try_new(words.clone())?;
        let trie_vec = da_dic.serialize().ok_or(Error::SerializeDictionary)?;
        Ok(Self {
            format_version: Self::CURRENT_FORMAT_VERSION.into(),
            words,
            trie_vec,
        })
    }
}

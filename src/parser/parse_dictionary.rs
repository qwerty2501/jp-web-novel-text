use crawdad::Trie;

use crate::parser::{DictionaryWordContainer, Error, Result};

pub struct DoubleArrayDictionary<WD>
where
    WD: DictionaryWordContainer,
{
    words: Vec<WD>,
    trie: Option<Trie>,
}

impl<WD> DoubleArrayDictionary<WD>
where
    WD: DictionaryWordContainer,
{
    pub fn new(words: Vec<WD>) -> Result<Self> {
        if words.is_empty() {
            Ok(Self { words, trie: None })
        } else {
            let trie = Trie::from_keys(words.iter().map(|w| w.word().key()))
                .map_err(Error::new_create_dictionary)?;
            Ok(Self {
                words,
                trie: Some(trie),
            })
        }
    }

    #[inline]
    pub fn get<I>(&self, key: I) -> Option<&WD>
    where
        I: IntoIterator<Item = char>,
    {
        if let Some(trie) = &self.trie {
            self.words.get(trie.exact_match(key)? as usize)
        } else {
            None
        }
    }
}

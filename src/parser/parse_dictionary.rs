use crawdad::Trie;

use crate::parser::{DictionaryWordContainer, Error, Result};

pub struct DoubleArrayDictionary<WD>
where
    WD: DictionaryWordContainer,
{
    words: Vec<WD>,
    trie: Trie,
}

impl<WD> DoubleArrayDictionary<WD>
where
    WD: DictionaryWordContainer,
{
    pub fn new(words: Vec<WD>) -> Result<Self> {
        let trie = Trie::from_keys(words.iter().map(|w| w.word().key()))
            .map_err(Error::new_create_dictionary)?;
        Ok(Self { words, trie })
    }

    #[inline]
    pub fn get<I>(&self, key: I) -> Option<&WD>
    where
        I: IntoIterator<Item = char>,
    {
        self.words.get(self.trie.exact_match(key)? as usize)
    }
}

use std::collections::HashMap;

use crawdad::Trie;

use crate::{
    dictionary::Word,
    parser::{Error, Result},
};

pub struct Dictionary<X> {
    words: Vec<Word<X>>,
    trie: Trie,
}

impl<X> Dictionary<X> {
    pub fn new(dic: HashMap<String, Word<X>>) -> Result<Self> {
        let trie = Trie::from_keys(dic.keys()).map_err(Error::new_create_dictionary)?;
        let words = dic.into_values().collect();
        Ok(Self { words, trie })
    }
    pub fn get<I>(&self, key: I) -> Option<&Word<X>>
    where
        I: IntoIterator<Item = char>,
    {
        self.words.get(self.trie.exact_match(key)? as usize)
    }
}

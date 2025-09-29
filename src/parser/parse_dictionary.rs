use std::collections::HashMap;

use crawdad::Trie;

use crate::{
    dictionary::Word,
    parser::{Error, Result},
};

pub struct Dictionary {
    words: Vec<Word>,
    trie: Trie,
}

impl Dictionary {
    pub fn new(dic: HashMap<String, Word>) -> Result<Self> {
        let trie = Trie::from_keys(dic.keys()).map_err(Error::new_create_dictionary)?;
        let words = dic.into_values().collect();
        Ok(Self { words, trie })
    }
    pub fn get<I>(&self, key: I) -> Option<&Word>
    where
        I: IntoIterator<Item = char>,
    {
        self.words.get(self.trie.exact_match(key)? as usize)
    }
}

use crawdad::Trie;

use crate::{
    dictionary::Word,
    parser::{Error, Result},
};

pub struct DoubleArrayDictionary<X> {
    words: Vec<Word<X>>,
    trie: Trie,
}

impl<X> DoubleArrayDictionary<X> {
    pub fn new(words: Vec<Word<X>>) -> Result<Self> {
        let trie =
            Trie::from_keys(words.iter().map(|w| w.key())).map_err(Error::new_create_dictionary)?;
        Ok(Self { words, trie })
    }

    #[inline]
    pub fn get<I>(&self, key: I) -> Option<&Word<X>>
    where
        I: IntoIterator<Item = char>,
    {
        self.words.get(self.trie.exact_match(key)? as usize)
    }
}

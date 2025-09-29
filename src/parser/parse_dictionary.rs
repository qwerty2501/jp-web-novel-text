use std::marker::PhantomData;

use crawdad::Trie;

use crate::{
    dictionary::Word,
    parser::{Error, Result},
};

pub struct Dictionary<S, X, WD> {
    words: Vec<WD>,
    trie: Trie,
    _x: PhantomData<X>,
    _s: PhantomData<S>,
}

impl<S, X, WD> Dictionary<S, X, WD>
where
    WD: WordContainer<S, X>,
{
    pub fn new(words: Vec<WD>) -> Result<Self> {
        let trie =
            Trie::from_keys(words.iter().map(|w| w.key())).map_err(Error::new_create_dictionary)?;
        Ok(Self {
            words,
            trie,
            _x: PhantomData,
            _s: PhantomData,
        })
    }
    pub fn get<I>(&self, key: I) -> Option<&WD>
    where
        I: IntoIterator<Item = char>,
    {
        self.words.get(self.trie.exact_match(key)? as usize)
    }
}

pub trait WordContainer<S, X>
where
    S: ?Sized,
{
    fn key(&self) -> &str;
    fn input_key(&self) -> &S;
    fn word(&self) -> &Word<X>;
}

impl<X> WordContainer<str, X> for Word<X> {
    fn key(&self) -> &str {
        self.key()
    }
    fn word(&self) -> &Word<X> {
        self
    }
    fn input_key(&self) -> &str {
        self.key()
    }
}

use std::collections::HashMap;

use nom::Input;

use crate::{
    DictionaryPhrase, Phrase,
    dictionary::Word,
    parser::{Result, parse_dictionary::Dictionary},
};

pub struct Parser<X> {
    dictionary: Dictionary<X>,
}

impl<X> Parser<X> {
    pub fn new(dic: HashMap<String, Word<X>>) -> Result<Parser<X>> {
        Ok(Parser {
            dictionary: Dictionary::<X>::new(dic)?,
        })
    }
}

pub struct ParseIter<'a, S, X> {
    text: &'a S,
    dictionary: &'a Dictionary<X>,
}

impl<X> Parser<X> {
    pub fn parse<'a, S>(&'a self, text: &'a S) -> ParseIter<'a, S, X> {
        ParseIter {
            text,
            dictionary: &self.dictionary,
        }
    }
}

impl<'a, S, X> Iterator for ParseIter<'a, S, X>
where
    &'a S: Input<Item = char>,
{
    type Item = Phrase<&'a S, &'a Word<X>>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(word) = self.dictionary.get(self.text.iter_elements()) {
            let (fragment, text) = self.text.take_split(word.key().len());
            self.text = text;
            Some(Phrase::new_dictionary_word(DictionaryPhrase::new(
                fragment, word,
            )))
        } else {
            unimplemented!()
        }
    }
}

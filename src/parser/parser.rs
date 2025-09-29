use std::collections::HashMap;

use nom::Input;

use crate::{
    DictionaryPhrase, Phrase,
    dictionary::Word,
    parser::{Result, parse_dictionary::Dictionary},
};

pub struct Parser {
    dictionary: Dictionary,
}

impl Parser {
    pub fn new(dic: HashMap<String, Word>) -> Result<Parser> {
        Ok(Parser {
            dictionary: Dictionary::new(dic)?,
        })
    }
}

pub struct ParseIter<'a, S> {
    text: &'a S,
    dictionary: &'a Dictionary,
}

impl Parser {
    pub fn parse<'a, S>(&'a self, text: &'a S) -> ParseIter<'a, S> {
        ParseIter {
            text,
            dictionary: &self.dictionary,
        }
    }
}

impl<'a, S> Iterator for ParseIter<'a, S>
where
    &'a S: Input,
{
    type Item = Phrase<&'a S, &'a Word>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(word) = self.dictionary.get(self.text) {
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

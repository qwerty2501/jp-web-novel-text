use nom::Input;

use crate::{
    DictionaryPhrase, Phrase,
    dictionary::Word,
    parser::{
        Result,
        parse_dictionary::{Dictionary, WordContainer},
    },
};

pub struct Parser<S, X, WD>
where
    WD: WordContainer<S, X>,
{
    dictionary: Dictionary<S, X, WD>,
}

impl<S, X, WD> Parser<S, X, WD>
where
    WD: WordContainer<S, X>,
{
    pub fn new(words: Vec<WD>) -> Result<Self> {
        Ok(Parser {
            dictionary: Dictionary::<S, X, WD>::new(words)?,
        })
    }
}
impl<'a, S, X, WD> Parser<S, X, WD>
where
    &'a S: Input<Item = char> + 'a,
    WD: WordContainer<S, X>,
{
    pub fn parse_iter(&'a self, text: &'a S) -> impl Iterator {
        ParseIter {
            text,
            dictionary: &self.dictionary,
        }
    }
}

pub struct ParseIter<'a, S, X, WD>
where
    &'a S: Input<Item = char> + 'a,
    WD: WordContainer<S, X>,
{
    text: &'a S,
    dictionary: &'a Dictionary<S, X, WD>,
}

impl<'a, S, X, WD> ParseIter<'a, S, X, WD>
where
    &'a S: Input<Item = char>,
    WD: WordContainer<S, X>,
{
    fn parse(&mut self) -> Option<Phrase<&'a S, &'a Word<X>>> {
        if let Some(wc) = self.dictionary.get(self.text.iter_elements()) {
            let (fragment, text) = self.text.take_split(wc.input_key_len());
            self.text = text;
            Some(Phrase::new_dictionary_word(DictionaryPhrase::new(
                fragment,
                wc.word(),
            )))
        } else {
            unimplemented!()
        }
    }
}

impl<'a, S, X, WD> Iterator for ParseIter<'a, S, X, WD>
where
    &'a S: Input<Item = char>,
    WD: WordContainer<S, X>,
{
    type Item = Phrase<&'a S, &'a Word<X>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.parse()
    }
}

use nom::Input;

use crate::{
    DictionaryPhrase, Phrase, PlainPhrase,
    dictionary::Word,
    parser::{
        Result,
        parse_dictionary::{DoubleArrayDictionary, WordContainer},
    },
};

pub struct Parser<S, X, WD>
where
    WD: WordContainer<S, X>,
{
    dictionary: DoubleArrayDictionary<S, X, WD>,
}

impl<S, X, WD> Parser<S, X, WD>
where
    WD: WordContainer<S, X>,
{
    pub fn new_with_dic(words: Vec<WD>) -> Result<Self> {
        Ok(Parser {
            dictionary: DoubleArrayDictionary::<S, X, WD>::new(words)?,
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
            plain_cache: None,
            next_phrase: None,
        }
    }
}

pub struct ParseIter<'a, S, X, WD>
where
    &'a S: Input<Item = char> + 'a,
    WD: WordContainer<S, X>,
{
    text: &'a S,
    dictionary: &'a DoubleArrayDictionary<S, X, WD>,
    plain_cache: Option<&'a S>,
    next_phrase: Option<Phrase<&'a S, &'a Word<X>>>,
}

impl<'a, S, X, WD> ParseIter<'a, S, X, WD>
where
    &'a S: Input<Item = char>,
    WD: WordContainer<S, X>,
{
    #[inline]
    fn parse_high_priority_once(&mut self) -> Option<(Phrase<&'a S, &'a Word<X>>, &'a S)> {
        unimplemented!()
    }

    #[inline]
    fn parse_once(&mut self) -> Option<(Phrase<&'a S, &'a Word<X>>, &'a S)> {
        if let Some(r) = self.parse_high_priority_once() {
            Some(r)
        } else {
            self.parse_dictionary_phrase_once()
        }
    }

    #[inline]
    fn parse_dictionary_phrase_once(&mut self) -> Option<(Phrase<&'a S, &'a Word<X>>, &'a S)> {
        if let Some(wc) = self.dictionary.get(self.text.iter_elements()) {
            let (fragment, text) = self.text.take_split(wc.input_key_len());
            Some((
                Phrase::new_dictionary_word(DictionaryPhrase::new(fragment, wc.word())),
                text,
            ))
        } else {
            None
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
        if let Some(next) = &self.next_phrase {
            let next = next.clone();
            self.next_phrase = None;
            Some(next)
        } else if let Some((phrase, next)) = self.parse_once() {
            self.text = next;
            if let Some(plain) = self.plain_cache {
                self.next_phrase = Some(phrase);
                Some(Phrase::new_plain(PlainPhrase::new(plain)))
            } else {
                Some(phrase)
            }
        } else {
            if self.plain_cache.is_none() {
                self.plain_cache = Some(self.text);
            }
            self.text = self.text.take_from(1);
            None
        }
    }
}

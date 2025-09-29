use nom::{AsBytes, Input};

use crate::{
    DictionaryPhrase, Phrase, PlainPhrase,
    dictionary::Word,
    parser::{Result, parse_dictionary::DoubleArrayDictionary},
};

pub struct GeneralBytesParser<X> {
    dictionary: DoubleArrayDictionary<X>,
}

impl<X> GeneralBytesParser<X> {
    pub fn new_with_dic(words: impl Into<Vec<Word<X>>>) -> Result<Self> {
        let words = words.into();
        Ok(GeneralBytesParser {
            dictionary: DoubleArrayDictionary::<X>::new(words)?,
        })
    }
}
impl<X> GeneralBytesParser<X> {
    pub fn parse_iter<'a, S>(&'a self, text: &'a S) -> impl Iterator
    where
        &'a S: Input<Item = char> + AsBytes + 'a,
    {
        GeneralBytesParseIter {
            text,
            dictionary: &self.dictionary,
            plain_cache: None,
            next_phrase: None,
        }
    }
}

pub struct GeneralBytesParseIter<'a, S, X>
where
    &'a S: Input<Item = char> + 'a,
{
    text: &'a S,
    dictionary: &'a DoubleArrayDictionary<X>,
    plain_cache: Option<&'a S>,
    next_phrase: Option<Phrase<&'a S, &'a Word<X>>>,
}

impl<'a, S, X> GeneralBytesParseIter<'a, S, X>
where
    &'a S: Input<Item = char>,
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
        if let Some(word) = self.dictionary.get(self.text.iter_elements()) {
            let (fragment, text) = self.text.take_split(word.key().len());
            Some((
                Phrase::new_dictionary_word(DictionaryPhrase::new(fragment, word)),
                text,
            ))
        } else {
            None
        }
    }
}

impl<'a, S, X> Iterator for GeneralBytesParseIter<'a, S, X>
where
    &'a S: Input<Item = char>,
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

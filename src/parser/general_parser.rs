use nom::{AsBytes, Input};

use crate::{
    DictionaryPhrase, Phrase, PlainPhrase,
    dictionary::Word,
    parser::{Result, parse_dictionary::DoubleArrayDictionary},
};

pub struct GeneralParser<WD>
where
    WD: WordContainer,
{
    dictionary: DoubleArrayDictionary<WD>,
}

pub(crate) struct GeneralParserGen;

impl GeneralParserGen {
    pub(crate) fn new_bytes_with_dic<X>(
        words: impl Into<Vec<Word<X>>>,
    ) -> Result<GeneralParser<Word<X>>> {
        let words = words.into();
        Ok(GeneralParser {
            dictionary: DoubleArrayDictionary::<Word<X>>::new(words)?,
        })
    }
}

impl<WD> GeneralParser<WD>
where
    WD: WordContainer,
{
    pub fn parse_iter<'a, S>(&'a self, text: &'a S) -> impl Iterator
    where
        &'a S: Input<Item = char> + AsBytes + 'a,
    {
        GeneralParseIter {
            text,
            dictionary: &self.dictionary,
            plain_cache: None,
            next_phrase: None,
        }
    }
}

pub struct GeneralParseIter<'a, S, WD>
where
    &'a S: Input<Item = char> + 'a,
    WD: WordContainer,
{
    text: &'a S,
    dictionary: &'a DoubleArrayDictionary<WD>,
    plain_cache: Option<&'a S>,
    next_phrase: Option<Phrase<&'a S, &'a WD>>,
}

impl<'a, S, WD> GeneralParseIter<'a, S, WD>
where
    &'a S: Input<Item = char>,
    WD: WordContainer,
{
    #[inline]
    fn parse_high_priority_once(&mut self) -> Option<(Phrase<&'a S, &'a WD>, &'a S)> {
        unimplemented!()
    }

    #[inline]
    fn parse_once(&mut self) -> Option<(Phrase<&'a S, &'a WD>, &'a S)> {
        if let Some(r) = self.parse_high_priority_once() {
            Some(r)
        } else {
            self.parse_dictionary_phrase_once()
        }
    }

    #[inline]
    fn parse_dictionary_phrase_once(&mut self) -> Option<(Phrase<&'a S, &'a WD>, &'a S)> {
        if let Some(word) = self.dictionary.get(self.text.iter_elements()) {
            let (fragment, text) = self.text.take_split(word.input_len());
            Some((
                Phrase::new_dictionary_word(DictionaryPhrase::new(fragment, word)),
                text,
            ))
        } else {
            None
        }
    }
}

impl<'a, S, WD> Iterator for GeneralParseIter<'a, S, WD>
where
    &'a S: Input<Item = char>,
    WD: WordContainer,
{
    type Item = Phrase<&'a S, &'a WD>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = &self.next_phrase {
            let next = next.clone();
            self.next_phrase = None;
            Some(next)
        } else if let Some((phrase, next)) = self.parse_once() {
            if let Some(plain) = self.plain_cache {
                let plain = plain.take(self.text.input_len() - plain.input_len());
                self.next_phrase = Some(phrase);
                self.text = next;
                self.plain_cache = None;
                Some(Phrase::new_plain(PlainPhrase::new(plain)))
            } else {
                self.text = next;
                Some(phrase)
            }
        } else {
            if self.plain_cache.is_none() {
                self.plain_cache = Some(self.text);
            }
            if self.text.input_len() == 0 {
                if let Some(plain) = self.plain_cache {
                    self.plain_cache = None;
                    Some(Phrase::new_plain(PlainPhrase::new(plain)))
                } else {
                    None
                }
            } else {
                self.text = self.text.take_from(1);
                None
            }
        }
    }
}

pub trait CharacterSize {}

pub struct ByteCharacterSize;
impl CharacterSize for ByteCharacterSize {}

pub trait WordContainer {
    type Extra;
    type CharacterSize: CharacterSize;
    fn word(&self) -> &Word<Self::Extra>;
    fn input_len(&self) -> usize;
}

impl<X> WordContainer for Word<X> {
    type Extra = X;
    type CharacterSize = ByteCharacterSize;
    fn word(&self) -> &Word<X> {
        self
    }
    fn input_len(&self) -> usize {
        self.key().len()
    }
}

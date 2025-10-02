use std::marker::PhantomData;

use nom::{Compare, Input, Parser, branch::alt};

use crate::{
    DictionaryPhrase, Phrase, PlainPhrase,
    dictionary::DictionaryWord,
    parser::{
        ParsedFlagment, Result,
        context_parser::ContextParser,
        nom_parsers::{new_line, space, tab, zenkaku_space},
        parse_dictionary::DoubleArrayDictionary,
    },
};

pub(crate) struct GeneralParser<WD>
where
    WD: DictionaryWordContainer,
{
    dictionary: DoubleArrayDictionary<WD>,
}

pub(crate) struct GeneralParserGen;

impl GeneralParserGen {
    pub(crate) fn new_bytes_with_dic<X>(
        words: impl Into<Vec<DictionaryWord<X>>>,
    ) -> Result<GeneralParser<DictionaryWord<X>>> {
        let words = words.into();
        Ok(GeneralParser {
            dictionary: DoubleArrayDictionary::<DictionaryWord<X>>::new(words)?,
        })
    }
}

impl<WD> GeneralParser<WD>
where
    WD: DictionaryWordContainer,
{
    pub fn parse_iter<'a, S, CP>(
        &'a self,
        text: S,
    ) -> impl Iterator<Item = ParsedFlagment<S, &'a WD>>
    where
        S: Input<Item = char> + Copy + Compare<&'static str>,
        CP: ContextParser<'a, S, WD>,
    {
        GeneralParseIter {
            text,
            dictionary: &self.dictionary,
            plain_cache: None,
            next_phrase: None,
            _cp: PhantomData::<CP>,
        }
    }
}

pub struct GeneralParseIter<'a, CP, S, WD>
where
    S: Input<Item = char> + Copy + Compare<&'static str>,
    WD: DictionaryWordContainer,
    CP: ContextParser<'a, S, WD>,
{
    text: S,
    dictionary: &'a DoubleArrayDictionary<WD>,
    plain_cache: Option<S>,
    next_phrase: Option<ParsedFlagment<S, &'a WD>>,
    _cp: PhantomData<CP>,
}

impl<'a, CP, S, WD> GeneralParseIter<'a, CP, S, WD>
where
    S: Input<Item = char> + Copy + Compare<&'static str>,
    WD: DictionaryWordContainer,
    CP: ContextParser<'a, S, WD>,
{
    #[inline]
    fn parse_high_priority_once(&mut self) -> Option<(S, ParsedFlagment<S, &'a WD>)> {
        alt((CP::parse, new_line, space, zenkaku_space, tab))
            .parse(self.text)
            .ok()
    }

    #[inline]
    fn parse_part_once(&mut self) -> Option<(S, ParsedFlagment<S, &'a WD>)> {
        if let Some(r) = self.parse_high_priority_once() {
            Some(r)
        } else {
            self.parse_dictionary_phrase_once()
        }
    }
    #[inline]
    fn parse_once(&mut self) -> (Option<ParsedFlagment<S, &'a WD>>, ParseStatus) {
        if let Some(next) = &self.next_phrase {
            let next = next.clone();
            self.next_phrase = None;
            (Some(next), ParseStatus::Progress)
        } else if let Some((next, phrase)) = self.parse_part_once() {
            if let Some(plain) = self.plain_cache {
                let plain = plain.take(plain.input_len() - self.text.input_len());
                self.next_phrase = Some(phrase);
                self.text = next;
                self.plain_cache = None;
                (
                    Some(ParsedFlagment::new(
                        plain,
                        Phrase::new_plain(PlainPhrase::new(plain)),
                    )),
                    ParseStatus::Progress,
                )
            } else {
                self.text = next;
                (Some(phrase), ParseStatus::Progress)
            }
        } else {
            if self.plain_cache.is_none() && self.text.input_len() > 0 {
                self.plain_cache = Some(self.text);
            }
            let mut i = self.text.iter_indices();
            i.next();
            if let Some((next_index, _)) = i.next() {
                self.text = self.text.take_from(next_index);
                (None, ParseStatus::Progress)
            } else if let Some(plain) = self.plain_cache {
                self.plain_cache = None;
                (
                    Some(ParsedFlagment::new(
                        plain,
                        Phrase::new_plain(PlainPhrase::new(plain)),
                    )),
                    ParseStatus::Progress,
                )
            } else {
                (None, ParseStatus::End)
            }
        }
    }

    #[inline]
    fn parse_dictionary_phrase_once(&mut self) -> Option<(S, ParsedFlagment<S, &'a WD>)> {
        if let Some(word) = self.dictionary.get(self.text) {
            let (fragment, text) = self.text.take_split(word.input_len());
            Some((
                text,
                ParsedFlagment::new(
                    fragment,
                    Phrase::new_dictionary_word(DictionaryPhrase::new(fragment, word)),
                ),
            ))
        } else {
            None
        }
    }
}

#[derive(PartialEq)]
enum ParseStatus {
    Progress,
    End,
}

impl<'a, CP, S, WD> Iterator for GeneralParseIter<'a, CP, S, WD>
where
    S: Input<Item = char> + Copy + Compare<&'static str>,
    WD: DictionaryWordContainer,
    CP: ContextParser<'a, S, WD>,
{
    type Item = ParsedFlagment<S, &'a WD>;
    fn next(&mut self) -> Option<Self::Item> {
        while let (phrase, status) = self.parse_once()
            && status == ParseStatus::Progress
        {
            if phrase.is_some() {
                return phrase;
            }
        }
        None
    }
}

pub trait CharacterSize {}

pub struct ByteCharacterSize;
impl CharacterSize for ByteCharacterSize {}

pub trait DictionaryWordContainer {
    type Extra;
    type CharacterSize: CharacterSize;
    fn word(&self) -> &DictionaryWord<Self::Extra>;
    fn input_len(&self) -> usize;
}

impl<X> DictionaryWordContainer for DictionaryWord<X> {
    type Extra = X;
    type CharacterSize = ByteCharacterSize;
    fn word(&self) -> &DictionaryWord<X> {
        self
    }
    fn input_len(&self) -> usize {
        self.key().len()
    }
}

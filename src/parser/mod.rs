mod context_parser;
mod general_parser;
mod nom_parsers;
mod parse_dictionary;

use derive_new::new;
pub use general_parser::*;
use nom::{AsBytes, Compare, Input};
use thiserror::Error;

use crate::{Phrase, dictionary::Word, parser::context_parser::GeneralContextParser};

#[derive(new, Error, Debug)]
pub enum Error {
    #[error("辞書作成に失敗しました")]
    CreateDictionary(crawdad::errors::CrawdadError),
}
pub type Result<T> = core::result::Result<T, Error>;

pub struct Parser<X>(GeneralParser<Word<X>>);

impl<X> Parser<X> {
    pub fn new_with_dic(words: impl Into<Vec<Word<X>>>) -> Result<Self> {
        Ok(Self(GeneralParserGen::new_bytes_with_dic(words)?))
    }
}

impl<X> Parser<X> {
    pub fn parse_iter<S>(&self, text: S) -> impl Iterator
    where
        S: Input<Item = char> + Copy + Compare<&'static str> + AsBytes,
    {
        self.0.parse_iter::<S, GeneralContextParser>(text)
    }
}

#[derive(new, Clone, PartialEq, Debug)]
#[new(visibility = "pub(crate)")]
pub struct ParsedFlagment<S, DW> {
    fragment: S,
    phrase: Phrase<S, DW>,
}

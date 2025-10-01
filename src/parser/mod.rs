mod context_parser;
mod general_parser;
mod nom_parsers;
mod parse_dictionary;

use derive_new::new;
use general_parser::*;
use nom::{AsBytes, Compare, Input};
use thiserror::Error;

use crate::{Phrase, dictionary::DictionaryWord, parser::context_parser::GeneralContextParser};

#[derive(new, Error, Debug)]
pub enum Error {
    #[error("辞書作成に失敗しました")]
    CreateDictionary(crawdad::errors::CrawdadError),
}
pub type Result<T> = core::result::Result<T, Error>;

pub struct Parser<X>(GeneralParser<DictionaryWord<X>>);

impl<X> Parser<X> {
    pub fn new() -> Result<Self> {
        Self::new_with_dic(vec![])
    }
    pub fn new_with_dic(words: impl Into<Vec<DictionaryWord<X>>>) -> Result<Self> {
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

#[cfg(test)]
mod tests {
    use crate::{RubyPhrase, RubyType};

    use super::*;
    use googletest::prelude::*;
    use rstest::*;

    #[fixture]
    fn words() -> Vec<DictionaryWord> {
        let r = RubyPhrase::new("foo", "ruby", RubyType::Instruction);
        let b: RubyPhrase<String> = r.to_permanent();
        vec![]
    }

    #[rstest]
    #[gtest]
    #[case()]
    fn parse_without_dic_works(
        #[case] text: &str,
        #[case] expected: Vec<ParsedFlagment<&str, &DictionaryWord>>,
    ) {
        let r = RubyPhrase::new("foo", "ruby", RubyType::Instruction);
        let f: Vec<ParsedFlagment<&str, &DictionaryWord>> = vec![];
        let ex = &f[0];
    }
}

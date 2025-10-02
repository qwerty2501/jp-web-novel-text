mod context_parser;
mod general_parser;
mod nom_parsers;
mod parse_dictionary;

use derive_getters::Getters;
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

pub struct Parser<X = ()>(GeneralParser<DictionaryWord<X>>);

impl Default for Parser<()> {
    fn default() -> Self {
        Self::try_new_with_dic(vec![]).unwrap()
    }
}

impl Parser {
    pub fn try_new_with_dic<X>(words: impl Into<Vec<DictionaryWord<X>>>) -> Result<Parser<X>> {
        Ok(Parser::<X>(GeneralParserGen::try_new_bytes_with_dic(
            words,
        )?))
    }
}

impl<X> Parser<X> {
    pub fn parse_iter<S>(
        &self,
        text: S,
    ) -> impl Iterator<Item = ParsedFlagment<S, &DictionaryWord<X>>>
    where
        S: Input<Item = char> + Copy + Compare<&'static str> + AsBytes,
    {
        self.0.parse_iter::<S, GeneralContextParser>(text)
    }
}

#[derive(new, Getters, Clone, PartialEq, Debug)]
#[new(visibility = "pub(crate)")]
pub struct ParsedFlagment<S, DW> {
    fragment: S,
    phrase: Phrase<S, DW>,
}

#[cfg(test)]
mod tests {

    use crate::{DictionaryPhrase, NewLinePhrase, PlainPhrase, RubyPhrase, RubyType};

    use super::*;
    use googletest::prelude::*;
    use rstest::*;

    #[fixture]
    fn words() -> Vec<DictionaryWord> {
        vec![DictionaryWord::new(
            "大砲".into(),
            "たいほう".into(),
            "foo".into(),
        )]
    }

    fn phrase_case1() -> Vec<ParsedFlagment<&'static str, &'static DictionaryWord>> {
        vec![
            ParsedFlagment::new(
                "大砲を撃て",
                Phrase::new_plain(PlainPhrase::new("大砲を撃て")),
            ),
            ParsedFlagment::new(
                "\n",
                Phrase::new_new_line(NewLinePhrase::new(crate::NewLineType::Lf)),
            ),
            ParsedFlagment::new(
                "|大砲(たいほう)",
                Phrase::new_ruby(RubyPhrase::new("大砲", "たいほう", RubyType::Instruction)),
            ),
            ParsedFlagment::new(
                "\n",
                Phrase::new_new_line(NewLinePhrase::new(crate::NewLineType::Lf)),
            ),
        ]
    }

    #[rstest]
    #[gtest]
    #[case(include_str!("test_data/parse_without_dic_works/case1.txt"), phrase_case1())]
    fn parse_without_dic_works(
        #[case] text: &str,
        #[case] expected: Vec<ParsedFlagment<&str, &DictionaryWord>>,
    ) {
        let parser = Parser::default();
        assert_that!(parser.parse_iter(text).collect::<Vec<_>>(), eq(&expected));
    }

    #[gtest]
    fn parse_with_dic() -> std::result::Result<(), Error> {
        let text = include_str!("test_data/parse_with_dic/case1.txt");
        let w = DictionaryWord::new("大砲".into(), "たいほう".into(), "foo".into());
        let expected: Vec<ParsedFlagment<&str, &DictionaryWord>> = vec![
            ParsedFlagment::new(
                "大砲",
                Phrase::new_dictionary_word(DictionaryPhrase::new("大砲", &w)),
            ),
            ParsedFlagment::new("を撃て", Phrase::new_plain(PlainPhrase::new("を撃て"))),
            ParsedFlagment::new(
                "\n",
                Phrase::new_new_line(NewLinePhrase::new(crate::NewLineType::Lf)),
            ),
            ParsedFlagment::new(
                "|大砲(たいほう)",
                Phrase::new_ruby(RubyPhrase::new("大砲", "たいほう", RubyType::Instruction)),
            ),
            ParsedFlagment::new(
                "\n",
                Phrase::new_new_line(NewLinePhrase::new(crate::NewLineType::Lf)),
            ),
        ];
        let dic_words = words();
        let parser = Parser::try_new_with_dic(dic_words)?;

        let actual = parser
            .parse_iter(text)
            .collect::<Vec<ParsedFlagment<&str, &DictionaryWord>>>();
        assert_that!(actual, eq(&expected));
        Ok(())
    }
}

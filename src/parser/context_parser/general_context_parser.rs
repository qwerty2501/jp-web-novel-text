use nom::{Compare, Input, Parser, branch::alt};

use crate::parser::{
    context_parser::ContextParser,
    nom_parsers::{kanji_ruby, ruby_instruction},
};

pub struct GeneralContextParser;

impl<'a, S, WD> ContextParser<'a, S, WD> for GeneralContextParser
where
    S: Input<Item = char> + Copy + Compare<&'static str>,
{
    fn parse(input: S) -> nom::IResult<S, crate::parser::ParsedFlagment<S, &'a WD>> {
        alt((ruby_instruction, kanji_ruby)).parse(input)
    }
}

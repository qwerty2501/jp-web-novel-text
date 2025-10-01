mod general_context_parser;
use nom::{Compare, IResult, Input};

use crate::parser::ParsedFlagment;

pub(crate) use general_context_parser::*;

pub trait ContextParser<'a, S, WD>
where
    S: Input<Item = char> + Copy + Compare<&'static str>,
{
    fn parse(input: S) -> IResult<S, ParsedFlagment<S, &'a WD>>;
}

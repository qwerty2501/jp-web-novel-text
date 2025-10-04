mod general_context_parser;
use nom::{Compare, IResult, Input};

use crate::parser::ParsedFragment;

pub(crate) use general_context_parser::*;

pub trait ContextParser<'a, S, WD>
where
    S: Input<Item = char> + Copy + Compare<&'static str>,
{
    fn parse(input: S) -> IResult<S, ParsedFragment<S, &'a WD>>;
}

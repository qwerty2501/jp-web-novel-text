use nom::{Compare, IResult, Input, Parser, character::complete::line_ending, combinator::map};

use crate::{NewLinePhrase, Phrase, parser::ParsedFlagment};

pub fn new_line<'a, S, DW>(input: S) -> IResult<S, ParsedFlagment<S, &'a DW>>
where
    S: Input<Item = char> + Compare<&'static str> + Copy,
{
    map(line_ending, |s| {
        ParsedFlagment::new(s, Phrase::new_new_line(NewLinePhrase::new(s)))
    })
    .parse(input)
}

#[cfg(test)]
mod tests {
    use crate::dictionary::Word;

    use super::*;
    use googletest::prelude::*;
    use nom::error;
    use rstest::*;

    #[gtest]
    #[rstest]
    #[case::rn("\r\n", Ok(("", ParsedFlagment::new("\r\n", Phrase::new_new_line(NewLinePhrase::new("\r\n"))))))]
    #[case::n("\n", Ok(("", ParsedFlagment::new("\n",Phrase::new_new_line(NewLinePhrase::new("\n"))))))]
    #[case::n_with_alpha("\naaa", Ok(("aaa",ParsedFlagment::new("\n",Phrase::new_new_line(NewLinePhrase::new("\n"))))))]
    #[case::n_with_kana("\nあいうえお", Ok(("あいうえお", ParsedFlagment::new("\n",Phrase::new_new_line(NewLinePhrase::new("\n"))))))]
    #[case::n_is_not_first(
        "aaaa\n",
        Err(nom::Err::Error(error::Error::new("aaaa\n", error::ErrorKind::CrLf)))
    )]
    fn new_line_works(
        #[case] input: &str,
        #[case] expected: IResult<&str, ParsedFlagment<&str, &Word>>,
    ) {
        assert_that!(new_line::<_, Word>(input), eq(&expected));
    }
}

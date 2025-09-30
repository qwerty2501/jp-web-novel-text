use nom::{Compare, IResult, Input, Parser, character::complete::line_ending, combinator::map};

use crate::{NewLinePhrase, Phrase};

pub fn new_line<'a, S, DW>(input: &'a S) -> IResult<&'a S, Phrase<&'a S, &'a DW>>
where
    S: ?Sized,
    &'a S: Input<Item = char> + Compare<&'static str> + 'a,
{
    map(line_ending, |s| Phrase::new_new_line(NewLinePhrase::new(s))).parse(input)
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
    #[case::rn("\r\n", Ok(("", Phrase::new_new_line(NewLinePhrase::new("\r\n")))))]
    #[case::n("\n", Ok(("", Phrase::new_new_line(NewLinePhrase::new("\n")))))]
    #[case::n_with_alpha("\naaa", Ok(("aaa", Phrase::new_new_line(NewLinePhrase::new("\n")))))]
    #[case::n_with_kana("\nあいうえお", Ok(("あいうえお", Phrase::new_new_line(NewLinePhrase::new("\n")))))]
    #[case::n_is_not_first(
        "aaaa\n",
        Err(nom::Err::Error(error::Error::new("aaaa\n", error::ErrorKind::CrLf)))
    )]
    fn new_line_works(#[case] input: &str, #[case] expected: IResult<&str, Phrase<&str, &Word>>) {
        let result = new_line::<_, Word>(input);
        assert_that!(result.is_ok(), eq(expected.is_ok()));
        if let Ok(r) = &result
            && let Ok(e) = &expected
        {
            assert_that!(r, eq(e));
        }

        if let Err(nom::Err::Error(re)) = &result
            && let Err(nom::Err::Error(ee)) = &expected
        {
            assert_that!(re, eq(ee));
        }
    }
}

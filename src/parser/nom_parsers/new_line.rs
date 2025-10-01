use nom::{Compare, IResult, Input, Parser, bytes::complete::tag};

use crate::{NewLinePhrase, NewLineType, Phrase, parser::ParsedFlagment};

pub(crate) fn new_line<'a, S, DW>(input: S) -> IResult<S, ParsedFlagment<S, &'a DW>>
where
    S: Input<Item = char> + Compare<&'static str> + Copy,
{
    if let Ok((input, nl)) = tag::<_, S, nom::error::Error<_>>("\n").parse(input) {
        Ok((
            input,
            ParsedFlagment::new(
                nl,
                Phrase::new_new_line(NewLinePhrase::new(NewLineType::Lf)),
            ),
        ))
    } else {
        let (input, nl) = tag("\r\n").parse(input)?;
        Ok((
            input,
            ParsedFlagment::new(
                nl,
                Phrase::new_new_line(NewLinePhrase::new(NewLineType::CrLf)),
            ),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::dictionary::DictionaryWord;

    use super::*;
    use googletest::prelude::*;
    use nom::error;
    use rstest::*;

    #[gtest]
    #[rstest]
    #[case::rn("\r\n", Ok(("", ParsedFlagment::new("\r\n", Phrase::new_new_line(NewLinePhrase::new(NewLineType::CrLf))))))]
    #[case::n("\n", Ok(("", ParsedFlagment::new("\n",Phrase::new_new_line(NewLinePhrase::new(NewLineType::Lf))))))]
    #[case::n_with_alpha("\naaa", Ok(("aaa",ParsedFlagment::new("\n",Phrase::new_new_line(NewLinePhrase::new(NewLineType::Lf))))))]
    #[case::n_with_kana("\nあいうえお", Ok(("あいうえお", ParsedFlagment::new("\n",Phrase::new_new_line(NewLinePhrase::new(NewLineType::Lf))))))]
    #[case::n_is_not_first(
        "aaaa\n",
        Err(nom::Err::Error(error::Error::new("aaaa\n", error::ErrorKind::Tag)))
    )]
    fn new_line_works(
        #[case] input: &str,
        #[case] expected: IResult<&str, ParsedFlagment<&str, &DictionaryWord>>,
    ) {
        assert_that!(new_line::<_, DictionaryWord>(input), eq(&expected));
    }
}

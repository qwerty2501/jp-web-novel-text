use nom::{
    IResult, Input, Parser,
    bytes::complete::{take_till1, take_while_m_n},
};

use crate::{
    Phrase, RubyPhrase,
    parser::nom_parsers::char::{
        is_end_ruby, is_new_line_escape, is_start_instruction, is_start_ruby,
    },
};

pub(crate) fn ruby<'a, S, DW>(input: S) -> IResult<S, Phrase<S, &'a DW>>
where
    S: Input<Item = char> + Copy,
{
    let (next, (_, target, _, ruby, _)) = (
        take_while_m_n(1, 1, is_start_instruction),
        take_till1(|c| is_start_ruby(c) || is_new_line_escape(c)),
        take_while_m_n(1, 1, is_start_ruby),
        take_till1(|c| is_end_ruby(c) || is_new_line_escape(c)),
        take_while_m_n(1, 1, is_end_ruby),
    )
        .parse(input)?;
    let fragment = input.take(input.input_len() - next.input_len());
    Ok((
        next,
        Phrase::new_ruby(RubyPhrase::new(fragment, target, ruby)),
    ))
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
    #[case("|玄人(くろうと)",Ok(("", Phrase::new_ruby(RubyPhrase::new("|玄人(くろうと)","玄人","くろうと")))))]
    #[case("|玄人《くろうと》",Ok(("", Phrase::new_ruby(RubyPhrase::new("|玄人《くろうと》","玄人","くろうと")))))]
    #[case("|玄人《くろうと)",Ok(("", Phrase::new_ruby(RubyPhrase::new("|玄人《くろうと)","玄人","くろうと")))))]
    #[case("|玄人(くろうと)ありうど",Ok(("ありうど", Phrase::new_ruby(RubyPhrase::new("|玄人(くろうと)","玄人","くろうと")))))]
    #[case(
        "あいうえお|玄人(くろうと)",
        Err(nom::Err::Error(error::Error::new(
            "あいうえお|玄人(くろうと)",
            error::ErrorKind::TakeWhileMN
        )))
    )]
    #[case(
        "|玄\n人(くろうと)",
        Err(nom::Err::Error(error::Error::new("\n人(くろうと)", error::ErrorKind::TakeWhileMN)))
    )]
    #[case(
        "|玄人\n(くろうと)",
        Err(nom::Err::Error(error::Error::new("\n(くろうと)", error::ErrorKind::TakeWhileMN)))
    )]
    #[case(
        "|玄人(くろ\nうと)",
        Err(nom::Err::Error(error::Error::new("\nうと)", error::ErrorKind::TakeWhileMN)))
    )]
    fn ruby_works(#[case] input: &str, #[case] expected: IResult<&str, Phrase<&str, &Word>>) {
        assert_that!(ruby(input), eq(&expected))
    }
}

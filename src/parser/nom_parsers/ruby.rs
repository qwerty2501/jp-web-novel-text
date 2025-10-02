use nom::{
    IResult, Input, Parser,
    bytes::complete::{take_till, take_till1, take_while, take_while_m_n},
    multi::many1_count,
    sequence::{delimited, preceded},
};

use crate::{
    Phrase, RubyPhrase, RubyType,
    parser::{
        ParsedFlagment,
        nom_parsers::char::{
            is_end_ruby, is_ideographic_variation_sequence, is_kanji, is_new_line_escape,
            is_start_instruction, is_start_ruby,
        },
    },
};

pub(crate) fn ruby_instruction<'a, S, DW>(input: S) -> IResult<S, ParsedFlagment<S, &'a DW>>
where
    S: Input<Item = char> + Copy,
{
    let (next, (target, ruby)) = (
        preceded(
            take_while_m_n(1, 1, is_start_instruction),
            take_till1(|c| is_start_ruby(c) || is_new_line_escape(c)),
        ),
        ruby,
    )
        .parse(input)?;
    let fragment = input.take(input.input_len() - next.input_len());
    Ok((
        next,
        ParsedFlagment::new(
            fragment,
            Phrase::new_ruby(RubyPhrase::new(target, ruby, RubyType::Instruction)),
        ),
    ))
}

fn ruby<S>(input: S) -> IResult<S, S>
where
    S: Input<Item = char> + Copy,
{
    delimited(
        take_while_m_n(1, 1, is_start_ruby),
        take_till(|c| is_end_ruby(c) || is_new_line_escape(c)),
        take_while_m_n(1, 1, is_end_ruby),
    )
    .parse(input)
}

pub(crate) fn kanji_ruby<'a, S, DW>(input: S) -> IResult<S, ParsedFlagment<S, &'a DW>>
where
    S: Input<Item = char> + Copy,
{
    let (next_input, _) = many1_count(kanji).parse(input)?;
    let kanji = input.take(input.input_len() - next_input.input_len());
    let (r, ruby) = ruby.parse(next_input)?;
    Ok((
        r,
        ParsedFlagment::new(
            input.take(input.input_len() - r.input_len()),
            Phrase::new_ruby(RubyPhrase::new(kanji, ruby, RubyType::KanjiWithRuby)),
        ),
    ))
}

fn kanji<S>(input: S) -> IResult<S, S>
where
    S: Input<Item = char> + Copy,
{
    let (r, _) = (
        take_while_m_n(1, 1, is_kanji),
        take_while(is_ideographic_variation_sequence),
    )
        .parse(input)?;
    Ok((r, input.take(input.input_len() - r.input_len())))
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
    #[case("|玄人(くろうと)",Ok(("", ParsedFlagment::new("|玄人(くろうと)",Phrase::new_ruby(RubyPhrase::new("玄人","くろうと",RubyType::Instruction))))))]
    #[case("|玄人《くろうと》",Ok(("", ParsedFlagment::new("|玄人《くろうと》",Phrase::new_ruby(RubyPhrase::new("玄人","くろうと",RubyType::Instruction))))))]
    #[case("|玄人《くろうと)",Ok(("", ParsedFlagment::new("|玄人《くろうと)",Phrase::new_ruby(RubyPhrase::new("玄人","くろうと",RubyType::Instruction))))))]
    #[case("|玄人《)",Ok(("", ParsedFlagment::new("|玄人《)",Phrase::new_ruby(RubyPhrase::new("玄人","",RubyType::Instruction))))))]
    #[case("|玄人(くろうと)ありうど",Ok(("ありうど", ParsedFlagment::new("|玄人(くろうと)",Phrase::new_ruby(RubyPhrase::new("玄人","くろうと",RubyType::Instruction))))))]
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
    fn ruby_instruction_works(
        #[case] input: &str,
        #[case] expected: IResult<&str, ParsedFlagment<&str, &DictionaryWord>>,
    ) {
        assert_that!(ruby_instruction(input), eq(&expected))
    }

    #[gtest]
    #[rstest]
    #[case("玄人(くろうと)",Ok(("", ParsedFlagment::new("玄人(くろうと)",Phrase::new_ruby(RubyPhrase::new("玄人","くろうと",RubyType::KanjiWithRuby))))))]
    #[case("玄人《くろうと》",Ok(("", ParsedFlagment::new("玄人《くろうと》",Phrase::new_ruby(RubyPhrase::new("玄人","くろうと",RubyType::KanjiWithRuby))))))]
    #[case("玄人《くろうと)",Ok(("", ParsedFlagment::new("玄人《くろうと)",Phrase::new_ruby(RubyPhrase::new("玄人","くろうと",RubyType::KanjiWithRuby))))))]
    #[case("玄人《)",Ok(("", ParsedFlagment::new("玄人《)",Phrase::new_ruby(RubyPhrase::new("玄人","",RubyType::KanjiWithRuby))))))]
    #[case("玄人(くろうと)ありうど",Ok(("ありうど", ParsedFlagment::new("玄人(くろうと)",Phrase::new_ruby(RubyPhrase::new("玄人","くろうと",RubyType::KanjiWithRuby))))))]
    #[case(
        "あいうえお|玄人(くろうと)",
        Err(nom::Err::Error(error::Error::new(
            "あいうえお|玄人(くろうと)",
            error::ErrorKind::Many1Count,
        )))
    )]
    #[case(
        "|玄人(くろうと)",
        Err(nom::Err::Error(error::Error::new("|玄人(くろうと)", error::ErrorKind::Many1Count)))
    )]
    #[case(
        "|玄人\n(くろうと)",
        Err(nom::Err::Error(error::Error::new(
            "|玄人\n(くろうと)",
            error::ErrorKind::Many1Count
        )))
    )]
    #[case(
        "玄人(くろ\nうと)",
        Err(nom::Err::Error(error::Error::new("\nうと)", error::ErrorKind::TakeWhileMN)))
    )]
    fn kanji_ruby_works(
        #[case] input: &str,
        #[case] expected: IResult<&str, ParsedFlagment<&str, &DictionaryWord>>,
    ) {
        assert_that!(kanji_ruby(input), eq(&expected))
    }

    #[gtest]
    #[rstest]
    #[case("葛󠄀",Ok(("", "葛󠄀")))]
    fn kanji_works(#[case] input: &str, #[case] expected: IResult<&str, &str>) {
        assert_that!(kanji(input), eq(&expected))
    }
}

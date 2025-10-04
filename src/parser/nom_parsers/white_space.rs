use nom::{Compare, IResult, Input, Parser, bytes::complete::take_while1, combinator::map};

use crate::{
    Phrase, WhiteSpacePhrase, WhiteSpaceType,
    parser::{
        ParsedFragment,
        nom_parsers::char::{is_space, is_tab, is_zenkaku_space},
    },
};
pub(crate) fn space<'a, S, DW>(input: S) -> IResult<S, ParsedFragment<S, &'a DW>>
where
    S: Input<Item = char> + Compare<&'static str> + Copy,
{
    map(take_while1(is_space), |s: S| {
        ParsedFragment::new(
            s,
            Phrase::new_white_space(WhiteSpacePhrase::new(s.input_len(), WhiteSpaceType::Space)),
        )
    })
    .parse(input)
}

pub(crate) fn zenkaku_space<'a, S, DW>(input: S) -> IResult<S, ParsedFragment<S, &'a DW>>
where
    S: Input<Item = char> + Compare<&'static str> + Copy,
{
    map(take_while1(is_zenkaku_space), |s: S| {
        ParsedFragment::new(
            s,
            Phrase::new_white_space(WhiteSpacePhrase::new(
                s.iter_elements().count(),
                WhiteSpaceType::ZenkakuSpace,
            )),
        )
    })
    .parse(input)
}

pub(crate) fn tab<'a, S, DW>(input: S) -> IResult<S, ParsedFragment<S, &'a DW>>
where
    S: Input<Item = char> + Compare<&'static str> + Copy,
{
    map(take_while1(is_tab), |s: S| {
        ParsedFragment::new(
            s,
            Phrase::new_white_space(WhiteSpacePhrase::new(s.input_len(), WhiteSpaceType::Tab)),
        )
    })
    .parse(input)
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
    #[case::space2("  ", Ok(("", ParsedFragment::new("  ", Phrase::new_white_space(WhiteSpacePhrase::new(2,WhiteSpaceType::Space))))))]
    #[case::space2_after_alpha("  aaa", Ok(("aaa", ParsedFragment::new("  ",Phrase::new_white_space(WhiteSpacePhrase::new(2,WhiteSpaceType::Space))))))]
    #[case::space2_after_kana("  あいうえお", Ok(("あいうえお", ParsedFragment::new("  ",Phrase::new_white_space(WhiteSpacePhrase::new(2,WhiteSpaceType::Space))))))]
    #[case::zenkaku_space(
        "　　",
        Err(nom::Err::Error(error::Error::new("　　", error::ErrorKind::TakeWhile1)))
    )]
    #[case::tab(
        "\t\t",
        Err(nom::Err::Error(error::Error::new("\t\t", error::ErrorKind::TakeWhile1)))
    )]
    #[case::space_before_alpha(
        "aaa  ",
        Err(nom::Err::Error(error::Error::new("aaa  ", error::ErrorKind::TakeWhile1)))
    )]
    fn space_works(
        #[case] input: &str,
        #[case] expected: IResult<&str, ParsedFragment<&str, &DictionaryWord>>,
    ) {
        assert_that!(space::<_, DictionaryWord>(input), eq(&expected));
    }

    #[gtest]
    #[rstest]
    #[case::zenkaku_space2("　　", Ok(("", ParsedFragment::new("　　", Phrase::new_white_space(WhiteSpacePhrase::new(2,WhiteSpaceType::ZenkakuSpace))))))]
    #[case::zenkaku_space2_after_alpha("　　aaa", Ok(("aaa",ParsedFragment::new("　　", Phrase::new_white_space(WhiteSpacePhrase::new(2,WhiteSpaceType::ZenkakuSpace))))))]
    #[case::zenkaku_space2_after_kana("　　あいうえお", Ok(("あいうえお",ParsedFragment::new("　　", Phrase::new_white_space(WhiteSpacePhrase::new(2,WhiteSpaceType::ZenkakuSpace))))))]
    #[case::space(
        "  ",
        Err(nom::Err::Error(error::Error::new("  ", error::ErrorKind::TakeWhile1)))
    )]
    #[case::tab(
        "\t\t",
        Err(nom::Err::Error(error::Error::new("\t\t", error::ErrorKind::TakeWhile1)))
    )]
    #[case::space_before_alpha(
        "aaa  ",
        Err(nom::Err::Error(error::Error::new("aaa  ", error::ErrorKind::TakeWhile1)))
    )]
    fn zenkaku_space_works(
        #[case] input: &str,
        #[case] expected: IResult<&str, ParsedFragment<&str, &DictionaryWord>>,
    ) {
        assert_that!(zenkaku_space::<_, DictionaryWord>(input), eq(&expected));
    }

    #[gtest]
    #[rstest]
    #[case::tab2("\t\t", Ok(("", ParsedFragment::new("\t\t",Phrase::new_white_space(WhiteSpacePhrase::new(2,WhiteSpaceType::Tab))))))]
    #[case::tab2_after_alpha("\t\taaa", Ok(("aaa",ParsedFragment::new("\t\t", Phrase::new_white_space(WhiteSpacePhrase::new(2,WhiteSpaceType::Tab))))))]
    #[case::tab2_after_kana("\t\tあいうえお", Ok(("あいうえお",ParsedFragment::new("\t\t", Phrase::new_white_space(WhiteSpacePhrase::new(2,WhiteSpaceType::Tab))))))]
    #[case::zenkaku_space(
        "　　",
        Err(nom::Err::Error(error::Error::new("　　", error::ErrorKind::TakeWhile1)))
    )]
    #[case::space(
        "  ",
        Err(nom::Err::Error(error::Error::new("  ", error::ErrorKind::TakeWhile1)))
    )]
    #[case::space_before_alpha(
        "aaa  ",
        Err(nom::Err::Error(error::Error::new("aaa  ", error::ErrorKind::TakeWhile1)))
    )]
    fn tab_works(
        #[case] input: &str,
        #[case] expected: IResult<&str, ParsedFragment<&str, &DictionaryWord>>,
    ) {
        assert_that!(tab::<_, DictionaryWord>(input), eq(&expected));
    }
}

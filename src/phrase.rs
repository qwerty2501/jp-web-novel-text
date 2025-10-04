use std::fmt::Display;

use derive_getters::Getters;
use derive_new::new;

use crate::DictionaryWord;

#[derive(new, Clone, PartialEq, Debug)]
pub enum Phrase<S = String, DW = DictionaryWord> {
    Ruby(RubyPhrase<S>),
    DictionaryWord(DictionaryPhrase<S, DW>),
    NewLine(NewLinePhrase),
    WhiteSpace(WhiteSpacePhrase),
    Plain(PlainPhrase<S>),
}

pub type PhraseRef<'a, S = str, DW = DictionaryWord> = Phrase<&'a S, &'a DW>;

impl<S: Display, DW> Display for Phrase<S, DW> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ruby(p) => p.fmt(f),
            Self::DictionaryWord(dw) => dw.fmt(f),
            Self::NewLine(nl) => nl.fmt(f),
            Self::WhiteSpace(ws) => ws.fmt(f),
            Self::Plain(pl) => pl.fmt(f),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum RubyType {
    Instruction,
    KanjiWithRuby,
}

#[derive(Getters, new, Clone, PartialEq, Debug)]
pub struct RubyPhrase<S> {
    target: S,
    ruby: S,
    ruby_type: RubyType,
}

impl<S: Display> Display for RubyPhrase<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.ruby_type {
            RubyType::Instruction => {
                f.write_str("|")?;
                self.target.fmt(f)?;
                f.write_str("《")?;
                self.ruby.fmt(f)?;
                f.write_str("》")
            }
            RubyType::KanjiWithRuby => {
                self.target.fmt(f)?;
                f.write_str("《")?;
                self.ruby.fmt(f)?;
                f.write_str("》")
            }
        }
    }
}

#[derive(Getters, new, Clone, PartialEq, Debug)]
pub struct DictionaryPhrase<S, DW> {
    target: S,
    word: DW,
}

impl<S: Display, DW> Display for DictionaryPhrase<S, DW> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.target.fmt(f)
    }
}

#[derive(Getters, new, Clone, PartialEq, Debug)]
pub struct PlainPhrase<S> {
    target: S,
}

impl<S: Display> Display for PlainPhrase<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.target.fmt(f)
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum NewLineType {
    Lf,
    CrLf,
}

#[derive(Getters, new, Clone, PartialEq, Debug)]
pub struct NewLinePhrase {
    new_line_type: NewLineType,
}
impl Display for NewLinePhrase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.new_line_type {
            NewLineType::Lf => f.write_str("\n"),
            NewLineType::CrLf => f.write_str("\r\n"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum WhiteSpaceType {
    Space,
    ZenkakuSpace,
    Tab,
}

#[derive(Getters, new, Clone, PartialEq, Debug)]
pub struct WhiteSpacePhrase {
    count: usize,
    white_space_type: WhiteSpaceType,
}

impl Display for WhiteSpacePhrase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self.white_space_type {
            WhiteSpaceType::Space => " ",
            WhiteSpaceType::Tab => "\t",
            WhiteSpaceType::ZenkakuSpace => "　",
        };
        for _ in 0..self.count {
            f.write_str(s)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::DictionaryWord;

    use super::*;
    use googletest::prelude::*;
    use rstest::*;

    #[gtest]
    #[rstest]
    #[case(WhiteSpacePhrase::new(3, WhiteSpaceType::Space), "   ")]
    #[case(WhiteSpacePhrase::new(4, WhiteSpaceType::ZenkakuSpace), "　　　　")]
    #[case(WhiteSpacePhrase::new(2, WhiteSpaceType::Tab), "\t\t")]
    fn white_space_phrase_display_works(#[case] ws: WhiteSpacePhrase, #[case] expected: &str) {
        assert_that!(ws.to_string(), eq(expected));
    }

    #[gtest]
    #[rstest]
    #[case(NewLinePhrase::new(NewLineType::Lf), "\n")]
    #[case(NewLinePhrase::new(NewLineType::CrLf), "\r\n")]
    fn new_line_phrase_display_works(#[case] nl: NewLinePhrase, #[case] expected: &str) {
        assert_that!(nl.to_string(), eq(expected));
    }

    #[gtest]
    #[rstest]
    #[case(PlainPhrase::<&str>::new("あいうえお"), "あいうえお")]
    #[case(PlainPhrase::<&str>::new("漢字"), "漢字")]
    fn plain_phrase_display_works(#[case] p: PlainPhrase<&str>, #[case] expected: &str) {
        assert_that!(p.to_string(), eq(expected));
    }
    #[gtest]
    #[rstest]
    #[case(DictionaryPhrase::<&str, DictionaryWord>::new("あいうえお",DictionaryWord::new("key".into(),"ruby".into(),"desc".into())), "あいうえお")]
    fn dictionary_phrase_display_works(
        #[case] dp: DictionaryPhrase<&str, DictionaryWord>,
        #[case] expected: &str,
    ) {
        assert_that!(dp.to_string(), eq(expected));
    }

    #[gtest]
    #[rstest]
    #[case(RubyPhrase::<&str>::new("あいうえお","ｱｲｳｴｵ",RubyType::Instruction), "|あいうえお《ｱｲｳｴｵ》")]
    #[case(RubyPhrase::<&str>::new("漢字","かんじ",RubyType::Instruction), "|漢字《かんじ》")]
    #[case(RubyPhrase::<&str>::new("漢字","かんじ",RubyType::KanjiWithRuby), "漢字《かんじ》")]
    fn ruby_phrase_display_works(#[case] p: RubyPhrase<&str>, #[case] expected: &str) {
        assert_that!(p.to_string(), eq(expected));
    }

    #[gtest]
    #[rstest]
    #[case(Phrase::new_ruby(RubyPhrase::<&str>::new("あいうえお","ｱｲｳｴｵ",RubyType::Instruction)), "|あいうえお《ｱｲｳｴｵ》")]
    #[case(Phrase::new_dictionary_word(DictionaryPhrase::<&str, DictionaryWord>::new("あいうえお",DictionaryWord::new("key".into(),"ruby".into(),"desc".into()))), "あいうえお")]
    #[case(Phrase::new_new_line(NewLinePhrase::new(NewLineType::Lf)), "\n")]
    #[case(
        Phrase::new_white_space(WhiteSpacePhrase::new(3, WhiteSpaceType::Tab)),
        "\t\t\t"
    )]
    #[case(Phrase::new_plain(PlainPhrase::<&str>::new("あいうえお")), "あいうえお")]
    fn phrase_display_works(#[case] p: Phrase<&str, DictionaryWord>, #[case] expected: &str) {
        assert_that!(p.to_string(), eq(expected));
    }
}

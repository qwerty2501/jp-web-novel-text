use derive_getters::Getters;
use derive_new::new;

#[derive(new, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Phrase<S, DW> {
    Ruby(RubyPhrase<S>),
    DictionaryWord(DictionaryPhrase<S, DW>),
    NewLine(NewLinePhrase),
    WhiteSpace(WhiteSpacePhrase),
    Plain(PlainPhrase<S>),
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RubyType {
    Instruction,
    KanjiWithRuby,
}

#[derive(Getters, new, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RubyPhrase<S> {
    target: S,
    ruby: S,
    ruby_type: RubyType,
}

#[derive(Getters, new, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DictionaryPhrase<S, DW> {
    target: S,
    word: DW,
}

#[derive(Getters, new, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PlainPhrase<S> {
    target: S,
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NewLineType {
    Lf,
    CrLf,
}

#[derive(Getters, new, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NewLinePhrase {
    new_line_type: NewLineType,
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WhiteSpaceType {
    Space,
    ZenkakuSpace,
    Tab,
}

#[derive(Getters, new, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WhiteSpacePhrase {
    count: usize,
    white_space_type: WhiteSpaceType,
}

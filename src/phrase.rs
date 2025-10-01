use derive_getters::Getters;
use derive_new::new;

#[derive(new, Clone, PartialEq, Debug)]
pub enum Phrase<S, DW> {
    Ruby(RubyPhrase<S>),
    DictionaryWord(DictionaryPhrase<S, DW>),
    NewLine(NewLinePhrase<S>),
    WhiteSpace(WhiteSpace),
    Plain(PlainPhrase<S>),
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

#[derive(Getters, new, Clone, PartialEq, Debug)]
pub struct DictionaryPhrase<S, DW> {
    target: S,
    word: DW,
}

#[derive(Getters, new, Clone, PartialEq, Debug)]
pub struct PlainPhrase<S> {
    target: S,
}

#[derive(Getters, new, Clone, PartialEq, Debug)]
pub struct NewLinePhrase<S> {
    target: S,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum WhiteSpaceType {
    Space,
    ZenkakuSpace,
    Tab,
}

#[derive(Getters, new, Clone, PartialEq, Debug)]
pub struct WhiteSpace {
    count: usize,
    white_space_type: WhiteSpaceType,
}

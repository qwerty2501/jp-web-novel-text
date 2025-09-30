use derive_getters::Getters;
use derive_new::new;

#[derive(new, Clone, PartialEq, Debug)]
pub enum Phrase<S, DW> {
    Ruby(RubyPhrase<S>),
    DictionaryWord(DictionaryPhrase<S, DW>),
    NewLine(NewLinePhrase<S>),
    WhiteSpace(WhiteSpace<S>),
    Plain(PlainPhrase<S>),
}

#[derive(Getters, new, Clone, PartialEq, Debug)]
pub struct RubyPhrase<S> {
    fragment: S,
    target: S,
    ruby: S,
}

#[derive(Getters, new, Clone, PartialEq, Debug)]
pub struct DictionaryPhrase<S, DW> {
    fragment: S,
    word: DW,
}

#[derive(Getters, new, Clone, PartialEq, Debug)]
pub struct PlainPhrase<S> {
    fragment: S,
}

#[derive(Getters, new, Clone, PartialEq, Debug)]
pub struct NewLinePhrase<S> {
    fragment: S,
}

#[derive(Clone, PartialEq, Debug)]
pub enum WhiteSpaceType {
    Space,
    ZenkakuSpace,
    Tab,
}

#[derive(Getters, new, Clone, PartialEq, Debug)]
pub struct WhiteSpace<S> {
    fragment: S,
    space_type: WhiteSpaceType,
}

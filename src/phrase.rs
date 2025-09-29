use derive_getters::Getters;
use derive_new::new;

#[derive(new)]
pub enum Phrase<S, DW> {
    Ruby(RubyPhrase<S>),
    DictionaryWord(DictionaryPhrase<S, DW>),
    NewLine(NewLinePhrase<S>),
    Plain(PlainPhrase<S>),
}

#[derive(Getters, new)]
pub struct RubyPhrase<S> {
    fragment: S,
    target: S,
    ruby: S,
}

#[derive(Getters, new)]
pub struct DictionaryPhrase<S, DW> {
    fragment: S,
    word: DW,
}

#[derive(Getters, new)]
pub struct PlainPhrase<S> {
    fragment: S,
}

#[derive(Getters, new)]
pub struct NewLinePhrase<S> {
    fragment: S,
}

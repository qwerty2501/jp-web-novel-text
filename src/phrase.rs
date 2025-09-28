use derive_getters::Getters;
use derive_new::new;

#[derive(new)]
pub enum Phrase<S, DW> {
    Ruby(RubyPhrase<S>),
    Dictionary(DictionaryPhrase<S, DW>),
    NewLine(NewLinePhrase<S>),
    Plain(PlainPhrase<S>),
}

#[derive(Getters, new)]
pub struct RubyPhrase<S> {
    phrase: S,
    target: S,
    ruby: S,
}

#[derive(Getters, new)]
pub struct DictionaryPhrase<S, DW> {
    phrase: S,
    dictionary: DW,
}

#[derive(Getters, new)]
pub struct PlainPhrase<S> {
    phrase: S,
}

#[derive(Getters, new)]
pub struct NewLinePhrase<S> {
    phrase: S,
}

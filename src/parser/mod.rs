mod parse_dictionary;
use derive_new::new;

use crate::{dictionary::Word, phrase::Phrase};

pub type ParsedPhrase<'a, S> = Phrase<&'a S, &'a Word>;

#[derive(new)]
pub enum Error {
    Dictionary(crawdad::errors::CrawdadError),
}
pub type Result<T> = core::result::Result<T, Error>;

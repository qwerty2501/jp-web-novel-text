mod general_parser;
mod parse_dictionary;
use derive_new::new;
pub use general_parser::*;
use thiserror::Error;

#[derive(new, Error, Debug)]
pub enum Error {
    #[error("辞書作成に失敗しました")]
    CreateDictionary(crawdad::errors::CrawdadError),
}
pub type Result<T> = core::result::Result<T, Error>;

pub type Parser<X> = GeneralBytesParser<X>;

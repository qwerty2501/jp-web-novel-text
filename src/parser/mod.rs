mod parse_dictionary;
mod parser;
use derive_new::new;
pub use parser::*;
use thiserror::Error;

#[derive(new, Error, Debug)]
pub enum Error {
    #[error("辞書作成に失敗しました")]
    CreateDictionary(crawdad::errors::CrawdadError),
}
pub type Result<T> = core::result::Result<T, Error>;

mod dictionary;
pub mod parser;
mod phrase;

pub use phrase::*;

use crate::{dictionary::Word, parser::GeneralParser};

pub type Parser<X> = GeneralParser<Word<X>>;

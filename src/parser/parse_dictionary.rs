use crawdad::Trie;
use nom::Input;

use crate::parser::{DictionaryWordContainer, Error, Result};

pub struct DoubleArrayDictionary<WD>
where
    WD: DictionaryWordContainer,
{
    words: Vec<WD>,
    trie: Option<Trie>,
}

impl<WD> DoubleArrayDictionary<WD>
where
    WD: DictionaryWordContainer,
{
    pub fn try_new(words: Vec<WD>) -> Result<Self> {
        if words.is_empty() {
            Ok(Self { words, trie: None })
        } else {
            let trie = Trie::from_keys(words.iter().map(|w| w.word().key()))
                .map_err(Error::new_create_dictionary)?;
            Ok(Self {
                words,
                trie: Some(trie),
            })
        }
    }

    #[inline]
    pub fn get<S>(&self, key: S) -> Option<&WD>
    where
        S: Input<Item = char>,
    {
        if let Some(trie) = &self.trie
            && let Some((i, _)) =
                trie.common_prefix_search(key.iter_elements())
                    .fold(None, |b, (i, length)| {
                        if let Some((bi, blength)) = b {
                            if length > blength {
                                Some((i, length))
                            } else {
                                Some((bi, blength))
                            }
                        } else {
                            Some((i, length))
                        }
                    })
        {
            self.words.get(i as usize)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::dictionary::DictionaryWord;

    use super::*;
    use googletest::prelude::*;
    use rstest::*;

    fn get_works_case1_words() -> Vec<DictionaryWord> {
        vec![
            DictionaryWord::new("炎".into(), "ほのお".into(), "火火".into()),
            DictionaryWord::new("炎炎".into(), "えんえん".into(), "火火火火".into()),
            DictionaryWord::new("延々".into(), "えんえん".into(), "えんえんえん".into()),
        ]
    }
    #[gtest]
    #[rstest]
    #[case("炎炎の炎", get_works_case1_words(), Some(DictionaryWord::new("炎炎".into(), "えんえん".into(), "火火火火".into())))]
    #[case("水水の水", get_works_case1_words(), None)]
    #[case("水炎炎の炎", get_works_case1_words(), None)]
    fn get_works(
        #[case] key: &str,
        #[case] words: Vec<DictionaryWord>,
        #[case] expected: Option<DictionaryWord>,
    ) {
        let dic = DoubleArrayDictionary::try_new(words).unwrap();
        assert_that!(dic.get(key), eq(expected.as_ref()))
    }
}

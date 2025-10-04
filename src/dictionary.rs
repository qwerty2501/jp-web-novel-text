use derive_getters::Getters;

use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::{
    Error, Result, general_parser::DictionaryWordContainer, parse_dictionary::DoubleArrayDictionary,
};

#[derive(Clone, new, PartialEq, Debug, Serialize, Deserialize)]
pub enum DictionaryWordKeyPhrase {
    Plain { target: String },
    Ruby { target: String, ruby: String },
}

#[derive(Getters, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct DictionaryWord<X = ()> {
    key: String,
    phrase: Vec<DictionaryWordKeyPhrase>,
    description: String,
    extra: X,
}

impl DictionaryWord {
    pub fn new(key: String, ruby: String, description: String) -> Self {
        Self::new_all(
            vec![if !ruby.is_empty() {
                DictionaryWordKeyPhrase::new_ruby(key, ruby)
            } else {
                DictionaryWordKeyPhrase::new_plain(key)
            }],
            description,
            (),
        )
    }
}

impl<X> DictionaryWord<X> {
    pub fn new_all(phrase: Vec<DictionaryWordKeyPhrase>, description: String, extra: X) -> Self {
        Self {
            key: phrase
                .iter()
                .map(|rp| match rp {
                    DictionaryWordKeyPhrase::Plain { target } => target.as_str(),
                    DictionaryWordKeyPhrase::Ruby { target, ruby: _ } => target.as_str(),
                })
                .collect(),
            phrase,
            description,
            extra,
        }
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct PreparedDictionary<WD>
where
    WD: Clone + DictionaryWordContainer,
{
    format_version: String,
    pub(crate) words: Vec<WD>,
    pub(crate) trie_vec: Vec<u8>,
}

impl<WD> PreparedDictionary<WD>
where
    WD: Clone + DictionaryWordContainer,
{
    pub(crate) const CURRENT_FORMAT_VERSION: &str = "1.0.0";
    pub(crate) fn format_version(&self) -> &str {
        &self.format_version
    }

    pub fn prepare(words: Vec<WD>) -> Result<Self> {
        let da_dic = DoubleArrayDictionary::try_new(words.clone())?;
        let trie_vec = da_dic.serialize().ok_or(Error::SerializeDictionary)?;
        Ok(Self {
            format_version: Self::CURRENT_FORMAT_VERSION.into(),
            words,
            trie_vec,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use googletest::prelude::*;
    use rstest::*;

    fn words() -> Vec<DictionaryWord> {
        vec![
        DictionaryWord::new_all(vec![
            DictionaryWordKeyPhrase::new_ruby("若々".into(),"わかわか".into()),
            DictionaryWordKeyPhrase::new_plain("しい".into()),
        ], "年齢のわりに若く見えること、また、活力や気力が衰えていないこと".into(),()),
        DictionaryWord::new("茶屋".into(), "ちゃや".into(), "製した茶を売る店。茶舗。".into()),
        DictionaryWord::new("自分".into(), "じぶん".into(), "その人自身。".into()),
        DictionaryWord::new("好奇心".into(), "こうきしん".into(), "新しいことや未知のこと、珍しい物事に対して強い興味や関心を持ち、それを知りたい、体験したいと思う心".into()),
        DictionaryWord::new(
            "旅館".into(),
            "りょかん".into(),
            "旅行者を泊めることを業とする家。やどや。".into(),
        ),
        DictionaryWord::new(
            "避暑地".into(),
            "ひしょち".into(),
            "夏の暑さを避けるために訪れる、涼しい気候の土地".into(),
        ),
        DictionaryWord::new("留守".into(), "るす".into(), "外出して家にいないこと。".into()),
        DictionaryWord::new(
            "どうして".into(),
            "".into(),
            "どのようなわけで。なぜ。".into(),
        ),
        DictionaryWord::new("様式".into(), "ようしき".into(), "ある範囲の事物・しかたに共通に認められる、一定のありかた。".into()),
        DictionaryWord::new("眼".into(), "め".into(), "物を見るはたらきをする器官。め。".into()),
        DictionaryWord::new("墓参り".into(), "はかまい".into(), "（自分の家や縁故者の）墓にお参りすること。".into()),
        DictionaryWord::new("交際".into(), "こうさい".into(), "人間（国）どうしがつきあうこと。つきあい。".into()),
        DictionaryWord::new("問答".into(), "もんどう".into(), "問うことと答えること。また、問い答えのやりとり。".into()),
    ]
    }

    #[gtest]
    #[rstest]
    #[case(words())]
    fn prepared_dictionary_prepare_works(#[case] words: Vec<DictionaryWord>) -> anyhow::Result<()> {
        let pd = PreparedDictionary::prepare(words.clone())?;
        assert_that!(pd.words, eq(&words));

        let serialized_data = serde_cbor::to_vec(&pd)?;
        let de_pd = serde_cbor::from_slice::<PreparedDictionary<DictionaryWord>>(&serialized_data)?;
        assert_that!(de_pd, eq(&pd));
        Ok(())
    }
}

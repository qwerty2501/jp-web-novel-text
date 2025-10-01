use derive_getters::Getters;

#[derive(Getters, Clone, PartialEq, Debug)]
pub struct DictionaryWord<X = ()> {
    key: String,
    ruby: String,
    description: String,
    extra: X,
}

impl DictionaryWord {
    pub fn new(key: String, ruby: String, description: String) -> Self {
        Self::new_extra(key, ruby, description, ())
    }
}

impl<X> DictionaryWord<X> {
    pub fn new_extra(key: String, ruby: String, description: String, extra: X) -> Self {
        Self {
            key,
            ruby,
            description,
            extra,
        }
    }
}

use derive_getters::Getters;

#[derive(Getters, Clone, PartialEq, Debug)]
pub struct Word<X = ()> {
    key: String,
    ruby: String,
    description: String,
    extra: X,
}

impl Word {
    pub fn new(key: String, ruby: String, description: String) -> Self {
        Self::new_extra(key, ruby, description, ())
    }
}

impl<X> Word<X> {
    pub fn new_extra(key: String, ruby: String, description: String, extra: X) -> Self {
        Self {
            key,
            ruby,
            description,
            extra,
        }
    }
}

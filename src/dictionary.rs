use derive_getters::Getters;

#[derive(Getters, Clone)]
pub struct Word {
    key: String,
    ruby: String,
    description: String,
}

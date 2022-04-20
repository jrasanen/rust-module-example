pub(crate) struct Address {
    pub(crate) street: String,
    pub(crate) city: String,
    pub(crate) state: String,
    pub(crate) zip: String,
}
pub(crate) struct Person {
    pub(crate) username: String,
    pub(crate) email: String,
    pub(crate) address: Address,
}

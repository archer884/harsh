pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// Error returned when the provided alphabet has insufficient distinct elements
    AlphabetLength,

    /// Error returned when a separator character is not found in the alphabet
    Separator,
}
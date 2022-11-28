use thiserror::Error;

#[derive(Debug, Clone, Copy, Error)]
pub enum Error {
    #[error("Failed to parse an identifier")]
    ParsingError,
}

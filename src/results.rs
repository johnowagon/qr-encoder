use std::fmt;

// Result and error type for Modes.
pub type ModeResult<T> = std::result::Result<T, ModeError>;
#[derive(Debug, Clone)]
pub struct ModeError;
impl fmt::Display for ModeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid mode operation.")
    }
}
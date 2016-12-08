use std::result;
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum GameError {
    GeneralError(String)
}

impl fmt::Display for GameError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GameError::GeneralError(ref msg) => write!(fmt, "{}", msg),
        }
    }
}
impl Error for GameError {
    fn description(&self) -> &str {
        match *self {
            GameError::GeneralError(ref msg) => msg,
        }
    }
}

type GameResult<T> = result::Result<T, GameError>;
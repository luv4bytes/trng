use std::array::TryFromSliceError;

/// Defines error types for tape errors.
#[derive(Debug)]
pub enum TapeErrorType {
    Index,
    Overflow,
    Io,
}

impl std::fmt::Display for TapeErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TapeErrorType::Index => {
                write!(f, "Index error")
            }
            TapeErrorType::Overflow => {
                write!(f, "Overflow error")
            }
            TapeErrorType::Io => {
                write!(f, "IO error")
            }
        }
    }
}

#[derive(Debug)]
pub struct TapeError {
    description: String,
    _type: TapeErrorType,
}

impl std::fmt::Display for TapeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tape Error [{}]: {}", self._type, self.description)
    }
}

impl std::error::Error for TapeError {}

impl From<std::io::Error> for TapeError {
    fn from(value: std::io::Error) -> Self {
        Self {
            description: value.to_string(),
            _type: TapeErrorType::Io,
        }
    }
}

impl From<TryFromSliceError> for TapeError {
    fn from(value: TryFromSliceError) -> Self {
        Self {
            description: value.to_string(),
            _type: TapeErrorType::Index,
        }
    }
}

impl TapeError {
    pub fn new(_type: TapeErrorType, description: String) -> TapeError {
        Self { _type, description }
    }
}

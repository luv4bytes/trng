use std::array::TryFromSliceError;

/// Defines error types for tape errors.
#[derive(Debug)]
pub enum TapeErrorType {
    IndexError,
    OverflowError,
    IoError,
}

impl std::fmt::Display for TapeErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TapeErrorType::IndexError => {
                write!(f, "{}", "Index error")
            }
            TapeErrorType::OverflowError => {
                write!(f, "{}", "Overflow error")
            }
            TapeErrorType::IoError => {
                write!(f, "{}", "IO error")
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
            _type: TapeErrorType::IoError,
        }
    }
}

impl From<TryFromSliceError> for TapeError {
    fn from(value: TryFromSliceError) -> Self {
        Self {
            description: value.to_string(),
            _type: TapeErrorType::IndexError,
        }
    }
}

impl TapeError {
    pub fn new(_type: TapeErrorType, description: String) -> TapeError {
        Self {
            _type: _type,
            description: description,
        }
    }
}

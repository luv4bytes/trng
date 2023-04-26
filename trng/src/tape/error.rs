// TRNG - Brainfucks pretty sister.
// Copyright (C) 2023 Lukas Pfeifer

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

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
impl TapeError {
    pub fn new(_type: TapeErrorType, description: String) -> TapeError {
        Self { _type, description }
    }
}

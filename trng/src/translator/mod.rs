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

mod error;

use self::error::TranslatorError;
use crate::lexer::{Lexer, TokenType};
use std::io::BufReader;

/// Type alias for a simple result with a `TranslatorError`.
type TranslatorResult<T> = Result<T, TranslatorError>;

/// Defines types of tokens used in Brainfuck.
#[derive(Debug, PartialEq)]
pub enum BrainfuckTokenType {
    /// Increment the pointer.
    GreaterThan,
    /// Decrement the pointer.
    LessThan,
    /// Increment the current cell value.
    Plus,
    /// Decrement the current cell value.
    Minus,
    /// Write the current cell value as an ASCII character to stdout.
    Dot,
    /// Read the next character from stdin and store it in the current cell.
    Comma,
    /// Start a loop.
    SqBrOpen,
    /// End a loop if the current cell value is 0.
    SqBrClosed,
}

/// Defines a token for Brainfuck.
#[derive(Debug, PartialEq)]
pub struct BrainfuckToken {
    _type: BrainfuckTokenType,
    value: String,
    line: u32,
    column: u32,
}

impl BrainfuckToken {
    pub fn new(_type: BrainfuckTokenType, value: String, line: u32, column: u32) -> Self {
        Self {
            _type,
            value,
            line,
            column,
        }
    }
}

/// Defines the translator for translating from trng to Brainfuck.
pub struct Translator;

impl Translator {
    pub fn translate(trng_code: &str) -> TranslatorResult<Vec<BrainfuckToken>> {
        let mut lexer = Lexer::new();
        let buf_reader = BufReader::new(trng_code.as_bytes());
        let tokens = lexer.tokenize(buf_reader)?;

        tokens
            .iter()
            .map(|token| {
                if !Translator::is_supported_token(&token._type) {
                    return Err(TranslatorError::new(
                        error::TranslatorErrorType::NotSupported,
                        format!(
                            "The token '{}' is not supported in translation.",
                            token.value
                        ),
                    ));
                }
                // TODO: Translate
                Ok(BrainfuckToken::new(
                    BrainfuckTokenType::Comma,
                    String::from("asdasd"),
                    2,
                    2,
                ))
            })
            .collect()
    }

    /// Checks if the given `TokenType` is supported in translation.
    fn is_supported_token(token_type: &TokenType) -> bool {
        matches!(
            token_type,
            TokenType::Dec
                | TokenType::Num
                | TokenType::Inc
                | TokenType::Pfw
                | TokenType::Pbw
                | TokenType::Wrt
                | TokenType::Rdi
                | TokenType::Lop
                | TokenType::Pol
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Translator;

    #[test]
    fn translate_is_empty_test() {
        let translated =
            Translator::translate("").expect("should have translated and returned an empty vector");
        assert_eq!(translated.len(), 0);
    }

    #[test]
    fn translate_is_not_empty_test() {
        let translated = Translator::translate("pfw 10")
            .expect("should have translated and returned a vector with 2 tokens");
        assert_ne!(translated.len(), 0)
    }
}

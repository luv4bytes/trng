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

use std::io::Read;

/// Defines data the lexer keeps track of.
pub struct Lexer {
    /// Current line number.
    current_line: u32,

    /// Current column number.
    current_column: u32,

    /// Token buffer for storing characters of tokens.
    token_buffer: String,
}

/// Defines token types.
pub enum TokenType {
    Pfw,
    Pbw,
    Inc,
    Dec,
    Num,
    Lop,
    Pol,
    Rdi,
    Rda,
    Wrt,
    Wrti8,
    Wrti16,
    Wrti32,
    Wrti64,
    Wrtu8,
    Wrtu16,
    Wrtu32,
    Wrtu64,
    Wrtf32,
    Wrtf64,
    Set,
    Wra,
    Clr,
    Unknown,
}

/// Defines a token.
pub struct Token {
    /// Token value.
    pub value: String,
    /// Line number of the token.
    pub line: u32,
    /// Column number of the token.
    pub column: u32,
    /// Column type.
    pub _type: TokenType,
}

impl Lexer {
    /// Constructor creates a new lexer with initialized values.
    pub fn new() -> Lexer {
        Lexer {
            current_column: 1,
            current_line: 1,
            token_buffer: String::new(),
        }
    }

    /// Tokenizes the given file and returns a vector of found tokens.
    /// # Arguments
    /// * `file` - Path of the file that will be tokenized.
    pub fn tokenize<T>(&mut self, read_from: T) -> Result<Vec<Token>, std::io::Error>
    where
        T: Read,
    {
        let mut tokens: Vec<Token> = vec![];

        for bres in read_from.bytes() {
            let byte = match bres {
                Ok(b) => b,
                Err(e) => return Err(e),
            };

            let byte_char = byte as char;

            if byte_char.is_whitespace() {
                let topt = self.token_from_buffer();
                match topt {
                    Some(token) => {
                        tokens.push(token);
                        self.token_buffer.clear();
                    }
                    None => (),
                }

                if byte_char == '\n' {
                    self.current_column = 1;
                    self.current_line += 1;
                } else {
                    self.current_column += 1;
                }

                continue;
            }
            self.token_buffer += &(byte as char).to_string();

            self.current_column += 1;
        }

        let topt = self.token_from_buffer();
        match topt {
            Some(token) => {
                tokens.push(token);
                self.token_buffer.clear();
            }
            None => (),
        }

        Ok(tokens)
    }

    /// Creates a token from the current buffer.
    fn token_from_buffer(&self) -> Option<Token> {
        if self.token_buffer.trim().is_empty() {
            return None;
        }

        let first_byte = self
            .token_buffer
            .as_bytes()
            .first()
            .expect("Token buffer should not have been empty at this point.");

        if (*first_byte as char).is_alphabetic() {
            return match &self.token_buffer.as_str() {
                &"pfw" => Some(self.token_from_internal(TokenType::Pfw)),
                &"pbw" => Some(self.token_from_internal(TokenType::Pbw)),
                &"inc" => Some(self.token_from_internal(TokenType::Inc)),
                &"dec" => Some(self.token_from_internal(TokenType::Dec)),
                &"lop" => Some(self.token_from_internal(TokenType::Lop)),
                &"pol" => Some(self.token_from_internal(TokenType::Pol)),
                &"wrt" => Some(self.token_from_internal(TokenType::Wrt)),
                &"wrti8" => Some(self.token_from_internal(TokenType::Wrti8)),
                &"wrti16" => Some(self.token_from_internal(TokenType::Wrti16)),
                &"wrti32" => Some(self.token_from_internal(TokenType::Wrti32)),
                &"wrti64" => Some(self.token_from_internal(TokenType::Wrti64)),
                &"wrtu8" => Some(self.token_from_internal(TokenType::Wrtu8)),
                &"wrtu16" => Some(self.token_from_internal(TokenType::Wrtu16)),
                &"wrtu32" => Some(self.token_from_internal(TokenType::Wrtu32)),
                &"wrtu64" => Some(self.token_from_internal(TokenType::Wrtu64)),
                &"wrtf32" => Some(self.token_from_internal(TokenType::Wrtf32)),
                &"wrtf64" => Some(self.token_from_internal(TokenType::Wrtf64)),
                &"rdi" => Some(self.token_from_internal(TokenType::Rdi)),
                &"rda" => Some(self.token_from_internal(TokenType::Rda)),
                &"set" => Some(self.token_from_internal(TokenType::Set)),
                &"wra" => Some(self.token_from_internal(TokenType::Wra)),
                &"clr" => Some(self.token_from_internal(TokenType::Clr)),
                &_ => Some(self.token_from_internal(TokenType::Unknown)),
            };
        } else if (*first_byte as char).is_numeric() {
            return Some(self.token_from_internal(TokenType::Num));
        } else {
            return Some(self.token_from_internal(TokenType::Unknown));
        }
    }

    fn token_from_internal(&self, _type: TokenType) -> Token {
        Token {
            value: self.token_buffer.to_string(),
            line: self.current_line,
            column: self.current_column - self.token_buffer.len() as u32,
            _type: _type,
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    pub fn tokenize_file_no_error_test() {
        let mut lexer = super::Lexer::new();

        let f = std::fs::File::open("../examples/example.trng").unwrap();

        let tokens = lexer.tokenize(f);

        assert!(!tokens.is_err());
    }
}
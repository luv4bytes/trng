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

use crate::{
    lexer,
    tape::{self, TapeError},
};
use std::io::Read;

/// Type alias for a simple result with an InterpreterError.
pub type InterpreterResult<T> = Result<T, InterpreterError>;

/// Defines an error that occurs during runtime.
#[derive(Debug)]
pub struct InterpreterError {
    pub description: String,
}

impl std::fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Interpreter Error: {}", self.description)
    }
}

impl std::error::Error for InterpreterError {}

impl From<TapeError> for InterpreterError {
    fn from(value: TapeError) -> Self {
        InterpreterError {
            description: value.to_string(),
        }
    }
}

/// Defines data for the interpreter.
pub struct Interpreter {
    /// The underlying tape.
    tape: tape::Tape,

    /// The index of the current instruciton.
    instruction_index: usize,

    loop_stack: Vec<usize>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self {
            tape: tape::Tape::new(30000),
            instruction_index: 0,
            loop_stack: vec![],
        }
    }
}

impl Interpreter {
    /// Constructor for creating a new interpreter.
    /// # Arguments
    /// * `data_tape_sz` - Size of the underlying tape.
    pub fn new(data_tape_sz: usize) -> Self {
        Self {
            tape: tape::Tape::new(data_tape_sz),
            instruction_index: 0,
            loop_stack: vec![],
        }
    }

    /// Resets the interpreter. This resets the tape and sets all indizes to 0.
    pub fn reset(&mut self) {
        self.tape.reset();
        self.instruction_index = 0;
        self.loop_stack = vec![];
    }

    /// Returns a reference to the data that is stored on the current tape.
    pub fn get_data(&self) -> &Vec<u8> {
        &self.tape.data
    }

    /// Starts the interpreter.
    /// # Arguments
    /// * `read_from` - Source for TRNG code. Can be anything that implements the `Read` trait.
    pub fn run<T>(&mut self, read_from: T) -> InterpreterResult<()>
    where
        T: Read,
    {
        let mut lexer = lexer::Lexer::new();

        let lex_result = lexer.tokenize(read_from);

        let tokens;

        match lex_result {
            Ok(t) => {
                tokens = t;
            }
            Err(e) => {
                return Err(InterpreterError {
                    description: e.to_string(),
                })
            }
        }

        let mut i = 0;
        while let Some(token) = tokens.get(i) {
            self.instruction_index = i;
            match token._type {
                lexer::TokenType::Pfw => {
                    i += 1;
                    let steps = expect_num::<usize>(tokens.get(i))?;
                    self.tape.pfw(steps)?;
                }
                lexer::TokenType::Pbw => {
                    i += 1;
                    let steps = expect_num::<usize>(tokens.get(i))?;
                    self.tape.pbw(steps)?;
                }
                lexer::TokenType::Inc => {
                    i += 1;
                    let by = expect_num::<u8>(tokens.get(i))?;
                    self.tape.inc(by)?;
                }
                lexer::TokenType::Dec => {
                    i += 1;
                    let by = expect_num::<u8>(tokens.get(i))?;
                    self.tape.dec(by)?;
                }
                lexer::TokenType::Lop => self.loop_stack.push(self.instruction_index),
                lexer::TokenType::Pol => {
                    let last = self.loop_stack.last();
                    match last {
                        Some(index) => {
                            let cur = self.tape.get_current_value();
                            match cur {
                                Ok(val) => {
                                    if val == 0 {
                                        self.loop_stack.pop(); // Index was valid and we don't need it any longer.
                                        i += 1;
                                        continue;
                                    }
                                }
                                Err(e) => return Err(InterpreterError::from(e)),
                            }

                            i = *index;
                        }
                        None => {
                            return Err(InterpreterError {
                                description: "Expected an index on the loop stack. Found nothing."
                                    .to_string(),
                            })
                        }
                    }
                }
                lexer::TokenType::Wrt => self.tape.wrt()?,
                lexer::TokenType::Wrti8 => self.tape.wrti8()?,
                lexer::TokenType::Wrti16 => self.tape.wrti16()?,
                lexer::TokenType::Wrti32 => self.tape.wrti32()?,
                lexer::TokenType::Wrti64 => self.tape.wrti64()?,
                lexer::TokenType::Wrtu8 => self.tape.wrtu8()?,
                lexer::TokenType::Wrtu16 => self.tape.wrtu16()?,
                lexer::TokenType::Wrtu32 => self.tape.wrtu32()?,
                lexer::TokenType::Wrtu64 => self.tape.wrtu64()?,
                lexer::TokenType::Wrtf32 => self.tape.wrtf32()?,
                lexer::TokenType::Wrtf64 => self.tape.wrtf64()?,
                lexer::TokenType::Rdi => self.tape.rdi()?,
                lexer::TokenType::Set => {
                    i += 1;
                    let value = tokens.get(i);
                    match value {
                        Some(t) => {
                            let set_res = self.tape.set(&t.value);
                            match set_res {
                                Ok(_) => (),
                                Err(e) => return Err(InterpreterError::from(e)),
                            }
                        }
                        None => {
                            return Err(InterpreterError {
                                description: "Expected a value. Found nothing.".to_string(),
                            })
                        }
                    }
                }
                lexer::TokenType::Seti8 => {
                    i += 1;
                    let v = expect_num::<i8>(tokens.get(i))?;
                    self.tape.seti8(v)?;
                }
                lexer::TokenType::Seti16 => {
                    i += 1;
                    let v = expect_num::<i16>(tokens.get(i))?;
                    self.tape.seti16(v)?;
                }
                lexer::TokenType::Seti32 => {
                    i += 1;
                    let v = expect_num::<i32>(tokens.get(i))?;
                    self.tape.seti32(v)?;
                }
                lexer::TokenType::Seti64 => {
                    i += 1;
                    let v = expect_num::<i64>(tokens.get(i))?;
                    self.tape.seti64(v)?;
                }
                lexer::TokenType::Setu8 => {
                    i += 1;
                    let v = expect_num::<u8>(tokens.get(i))?;
                    self.tape.setu8(v)?;
                }
                lexer::TokenType::Setu16 => {
                    i += 1;
                    let v = expect_num::<u16>(tokens.get(i))?;
                    self.tape.setu16(v)?;
                }
                lexer::TokenType::Setu32 => {
                    i += 1;
                    let v = expect_num::<u32>(tokens.get(i))?;
                    self.tape.setu32(v)?;
                }
                lexer::TokenType::Setu64 => {
                    i += 1;
                    let v = expect_num::<u64>(tokens.get(i))?;
                    self.tape.setu64(v)?;
                }
                lexer::TokenType::Setf32 => {
                    i += 1;
                    let v = expect_num::<f32>(tokens.get(i))?;
                    self.tape.setf32(v)?;
                }
                lexer::TokenType::Setf64 => {
                    i += 1;
                    let v = expect_num::<f64>(tokens.get(i))?;
                    self.tape.setf64(v)?;
                }
                lexer::TokenType::Wra => self.tape.wra()?,
                lexer::TokenType::Rda => self.tape.rda()?,
                lexer::TokenType::Clr => self.tape.clr()?,
                lexer::TokenType::Unknown => {
                    return Err(InterpreterError {
                        description: format!(
                            "Found unknown token '{}'. - ln: {}, col: {}",
                            token.value, token.line, token.column
                        ),
                    })
                }
                _ => {
                    return Err(InterpreterError {
                        description: "Unexpected error.".to_string(),
                    })
                }
            }

            i += 1;
        }

        Ok(())
    }
}

/// Expects the given `Token` to be a `Num` token.
/// Returns an error if the token is different from what was expected.
/// # Arguments
/// * `token` - Option of a token.
fn expect_num<T>(token: Option<&lexer::Token>) -> InterpreterResult<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    match token {
        Some(tok) => match tok._type {
            lexer::TokenType::Num => {
                let num = tok.value.parse::<T>();
                match num {
                    Ok(n) => Ok(n),
                    Err(e) => Err(InterpreterError {
                        description: e.to_string(),
                    }),
                }
            }
            _ => Err(InterpreterError {
                description: format!(
                    "Expected 'num'. Found '{}' - ln: {}, col: {}",
                    tok.value, tok.line, tok.column
                ),
            }),
        },
        None => Err(InterpreterError {
            description: "Expected 'num'. Found nothing instead.".to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    #[test]
    fn wrt_is_successful_test() {
        let code = r#"
            set Hello
            pbw 5
            wrt
            pfw 1
            wrt
            pfw 1
            wrt
            pfw 1
            wrt
            pfw 1
            wrt
            "#;

        let reader = BufReader::new(code.as_bytes());
        let mut interpreter = super::Interpreter::default();
        let run_result = interpreter.run(reader);

        assert!(!run_result.is_err());
    }

    #[test]
    fn wra_is_successful_test() {
        let code = "
            set Hello
            pbw 5
            wra";

        let reader = BufReader::new(code.as_bytes());
        let mut interpreter = super::Interpreter::default();
        let run_result = interpreter.run(reader);

        assert!(!run_result.is_err());
    }

    #[test]
    fn run_file_is_successful_test() {
        let f = std::fs::File::open("../examples/example.trng");

        assert!(!f.is_err());

        let mut interpreter = super::Interpreter::default();
        let run_result = interpreter.run(f.unwrap());

        assert!(!run_result.is_err());
    }

    #[test]
    fn run_loop_example_successful_test() {
        let f = std::fs::File::open("../examples/loop.trng");

        assert!(!f.is_err());

        let mut interpreter = super::Interpreter::default();
        let run_result = interpreter.run(f.unwrap());

        assert!(!run_result.is_err());
    }

    #[test]
    fn run_seti_example_successful_test() {
        let f = std::fs::File::open("../examples/seti.trng");

        assert!(!f.is_err());

        let mut interpreter = super::Interpreter::default();
        let run_result = interpreter.run(f.unwrap());

        assert!(!run_result.is_err());
    }

    #[test]
    fn run_setu_example_successful_test() {
        let f = std::fs::File::open("../examples/setu.trng");

        assert!(!f.is_err());

        let mut interpreter = super::Interpreter::default();
        let run_result = interpreter.run(f.unwrap());

        assert!(!run_result.is_err());
    }

    #[test]
    fn run_setf_example_successful_test() {
        let f = std::fs::File::open("../examples/setf.trng");

        assert!(!f.is_err());

        let mut interpreter = super::Interpreter::default();
        let run_result = interpreter.run(f.unwrap());

        assert!(!run_result.is_err());
    }

    #[test]
    fn reset_is_successful_test() {
        let f = std::fs::File::open("../examples/example.trng");

        assert!(!f.is_err());

        let mut interpreter = super::Interpreter::default();
        let run_result = interpreter.run(f.unwrap());

        assert!(!run_result.is_err());

        interpreter.reset();

        assert_eq!(interpreter.get_data()[0], 0)
    }
}

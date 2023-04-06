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

use std::io::{Read, Write};

use super::TapeError;

/// Simulates the tape for TRNG.
pub struct Tape {
    pub data: Vec<u8>,
    pub ptr_index: usize,
}

impl Default for Tape {
    fn default() -> Self {
        Self {
            data: vec![0; 30000],
            ptr_index: 0,
        }
    }
}

impl Tape {
    /// Function returns a new tape.
    ///
    /// The data band will have data_band_sz cells.
    ///
    /// * `data_band_sz` - The number of cells on the data band.
    pub fn new(data_band_sz: usize) -> Self {
        Self {
            data: vec![0; data_band_sz],
            ptr_index: 0,
        }
    }

    /// Resets all cells of the tape to 0.
    pub fn reset(&mut self) {
        for c in &mut self.data {
            *c = 0;
        }
        self.ptr_index = 0;
    }

    /// Gets the value of the current cell.
    pub fn get_current_value(&mut self) -> Result<u8, TapeError> {
        let cur = self.data.get(self.ptr_index);

        match cur {
            Some(val) => Ok(*val),
            None => Err(TapeError::new(
                super::TapeErrorType::IndexError,
                format!(
                    "Getting the current value at pointer index {} is invalid.",
                    self.ptr_index
                ),
            )),
        }
    }

    /// Moves the pointer (read/write head) forward.
    ///
    /// * `steps` - The number of steps to move forward on the tape.
    pub fn pfw(&mut self, steps: usize) -> Result<(), TapeError> {
        if self.ptr_index + steps >= self.data.capacity() {
            return Err(TapeError::new(
                super::TapeErrorType::IndexError,
                format!(
                    "Moving the pointer {} step(s) forward would result in overshooting the tape.",
                    steps
                ),
            ));
        }

        self.ptr_index = self.ptr_index + steps;
        Ok(())
    }

    /// Moves the pointer (read/write head) backward.
    ///
    /// * `steps` - The number of steps to move backward on the tape.
    pub fn pbw(&mut self, steps: usize) -> Result<(), TapeError> {
        let subbed = self.ptr_index.checked_sub(steps);

        match subbed {
            Some(n) => Ok(self.ptr_index = n),
            None => Err(TapeError::new(
                super::TapeErrorType::IndexError,
                format!(
                    "Moving the pointer {} step(s) backward would result in overshooting the tape.",
                    steps
                ),
            )),
        }
    }

    /// Increments the value of the current cell
    ///
    /// * `by` - This value gets added to the value of the current cell.
    pub fn inc(&mut self, by: u8) -> Result<(), TapeError> {
        let added = self.get_current_value()?.checked_add(by);

        match added {
            Some(n) => Ok(self.data[self.ptr_index] = n),
            None => Err(TapeError::new(
                super::TapeErrorType::OverflowError,
                format!(
                    "Adding {} to the current cell value would result in an overflow.",
                    by
                ),
            )),
        }
    }

    /// Decrements the value of the current cell
    ///
    /// * `by` - This value gets subtracted from the value of the current cell.
    pub fn dec(&mut self, by: u8) -> Result<(), TapeError> {
        let subbed = self.get_current_value()?.checked_sub(by);

        match subbed {
            Some(n) => Ok(self.data[self.ptr_index] = n),
            None => Err(TapeError::new(
                super::TapeErrorType::OverflowError,
                format!(
                    "Subtracting {} from the current cell value would result in an overflow.",
                    by
                ),
            )),
        }
    }

    /// Writes the value of the current cell to stdout.
    pub fn wrt(&mut self) -> Result<(), TapeError> {
        let write_res = std::io::stdout()
            .lock()
            .write_all(&[self.get_current_value()?]);

        match write_res {
            Ok(_) => Ok(()),
            Err(e) => Err(TapeError::from(e)),
        }
    }

    /// Writes the value of the current cell as an 8-bit signed integer to stdout.
    pub fn wrti8(&mut self) -> Result<(), TapeError> {
        let b = self.get_current_value()? as i8;
        let i = b.to_string();

        let write_res = std::io::stdout().lock().write_all(i.as_bytes());

        match write_res {
            Ok(_) => Ok(()),
            Err(e) => Err(TapeError::from(e)),
        }
    }

    /// Writes the current cell and the next interpreted as an 16-bit signed integer to stdout.
    pub fn wrti16(&mut self) -> Result<(), TapeError> {
        let slice = &self.data[self.ptr_index..self.ptr_index + 2];

        let byte_slice = <&[u8; 2]>::try_from(slice);
        let bytes = match byte_slice {
            Ok(b) => b,
            Err(e) => return Err(TapeError::from(e)),
        };

        let resi16 = i16::from_be_bytes(*bytes);

        let s = resi16.to_string();

        let write_res = std::io::stdout().lock().write_all(s.as_bytes());

        match write_res {
            Ok(_) => Ok(()),
            Err(e) => Err(TapeError::from(e)),
        }
    }

    /// Writes the current cell and the next three interpreted as an 32-bit signed integer to stdout.
    pub fn wrti32(&mut self) -> Result<(), TapeError> {
        let slice = &self.data[self.ptr_index..self.ptr_index + 4];

        let byte_slice = <&[u8; 4]>::try_from(slice);
        let bytes = match byte_slice {
            Ok(b) => b,
            Err(e) => return Err(TapeError::from(e)),
        };

        let resi32 = i32::from_be_bytes(*bytes);

        let s = resi32.to_string();

        let write_res = std::io::stdout().lock().write_all(s.as_bytes());

        match write_res {
            Ok(_) => Ok(()),
            Err(e) => Err(TapeError::from(e)),
        }
    }

    /// Writes the current cell and the next seven interpreted as an 64-bit signed integer to stdout.
    pub fn wrti64(&mut self) -> Result<(), TapeError> {
        let slice = &self.data[self.ptr_index..self.ptr_index + 8];

        let byte_slice = <&[u8; 8]>::try_from(slice);
        let bytes = match byte_slice {
            Ok(b) => b,
            Err(e) => return Err(TapeError::from(e)),
        };

        let resi64 = i64::from_be_bytes(*bytes);

        let s = resi64.to_string();

        let write_res = std::io::stdout().lock().write_all(s.as_bytes());

        match write_res {
            Ok(_) => Ok(()),
            Err(e) => Err(TapeError::from(e)),
        }
    }

    /// Writes the value of the current cell as an 8-bit unsigned integer to stdout.
    pub fn wrtu8(&mut self) -> Result<(), TapeError> {
        let b = self.get_current_value()?;
        let i = b.to_string();

        let write_res = std::io::stdout().lock().write_all(i.as_bytes());

        match write_res {
            Ok(_) => Ok(()),
            Err(e) => Err(TapeError::from(e)),
        }
    }

    /// Writes the current cell and the next interpreted as an 16-bit unsigned integer to stdout.
    pub fn wrtu16(&mut self) -> Result<(), TapeError> {
        let slice = &self.data[self.ptr_index..self.ptr_index + 2];

        let byte_slice = <&[u8; 2]>::try_from(slice);
        let bytes = match byte_slice {
            Ok(b) => b,
            Err(e) => return Err(TapeError::from(e)),
        };

        let resu16 = u16::from_be_bytes(*bytes);

        let s = resu16.to_string();

        let write_res = std::io::stdout().lock().write_all(s.as_bytes());

        match write_res {
            Ok(_) => Ok(()),
            Err(e) => Err(TapeError::from(e)),
        }
    }

    /// Writes the current cell and the next three interpreted as an 32-bit signed integer to stdout.
    pub fn wrtu32(&mut self) -> Result<(), TapeError> {
        let slice = &self.data[self.ptr_index..self.ptr_index + 4];

        let byte_slice = <&[u8; 4]>::try_from(slice);
        let bytes = match byte_slice {
            Ok(b) => b,
            Err(e) => return Err(TapeError::from(e)),
        };

        let resu32 = u32::from_be_bytes(*bytes);

        let s = resu32.to_string();

        let write_res = std::io::stdout().lock().write_all(s.as_bytes());

        match write_res {
            Ok(_) => Ok(()),
            Err(e) => Err(TapeError::from(e)),
        }
    }

    /// Writes the current cell and the next seven interpreted as an 64-bit unsigned integer to stdout.
    pub fn wrtu64(&mut self) -> Result<(), TapeError> {
        let slice = &self.data[self.ptr_index..self.ptr_index + 8];

        let byte_slice = <&[u8; 8]>::try_from(slice);
        let bytes = match byte_slice {
            Ok(b) => b,
            Err(e) => return Err(TapeError::from(e)),
        };

        let resu64 = u64::from_be_bytes(*bytes);

        let s = resu64.to_string();

        let write_res = std::io::stdout().lock().write_all(s.as_bytes());

        match write_res {
            Ok(_) => Ok(()),
            Err(e) => Err(TapeError::from(e)),
        }
    }

    /// Writes the current cell and the next three interpreted as an 32-bit floating point number to stdout.
    pub fn wrtf32(&mut self) -> Result<(), TapeError> {
        let slice = &self.data[self.ptr_index..self.ptr_index + 4];

        let byte_slice = <&[u8; 4]>::try_from(slice);
        let bytes = match byte_slice {
            Ok(b) => b,
            Err(e) => return Err(TapeError::from(e)),
        };

        let resf32 = f32::from_be_bytes(*bytes);

        let s = resf32.to_string();

        let write_res = std::io::stdout().lock().write_all(s.as_bytes());

        match write_res {
            Ok(_) => Ok(()),
            Err(e) => Err(TapeError::from(e)),
        }
    }

    /// Writes the current cell and the next seven interpreted as an 64-bit floating point number to stdout.
    pub fn wrtf64(&mut self) -> Result<(), TapeError> {
        let slice = &self.data[self.ptr_index..self.ptr_index + 8];

        let byte_slice = <&[u8; 8]>::try_from(slice);
        let bytes = match byte_slice {
            Ok(b) => b,
            Err(e) => return Err(TapeError::from(e)),
        };

        let resf64 = f64::from_be_bytes(*bytes);

        let s = resf64.to_string();

        let write_res = std::io::stdout().lock().write_all(s.as_bytes());

        match write_res {
            Ok(_) => Ok(()),
            Err(e) => Err(TapeError::from(e)),
        }
    }

    /// Reads a character from stdin and stores it in the current cell.
    pub fn rdi(&mut self) -> Result<(), TapeError> {
        let b = std::io::stdin().lock().bytes().next();
        match b {
            Some(res) => match res {
                Ok(byte) => Ok(self.store(byte)?),
                Err(e) => Err(TapeError::from(e)),
            },
            None => Ok(()),
        }
    }

    /// Sets the given value, placing each byte in a separate cell and incrementing the pointer accordingly.
    /// * `v` - The value to set.
    pub fn set(&mut self, v: &str) -> Result<(), TapeError> {
        for b in v.as_bytes() {
            self.data[self.ptr_index] = *b;
            self.ptr_index += 1;
        }

        Ok(())
    }

    /// Writes the current cell and all following cells to stdout until a null byte is encountered.
    /// The pointer is incremented accordingly.
    pub fn wra(&mut self) -> Result<(), TapeError> {
        loop {
            let b = self.get_current_value()?;

            if b == 0 {
                break;
            }

            match std::io::stdout().lock().write_all(&[b]) {
                Ok(_) => {
                    self.step_fw()?;
                }
                Err(e) => return Err(TapeError::from(e)),
            }
        }

        Ok(())
    }

    /// Reads all bytes from stdin until LF is encountered.
    /// The pointer is incremented accordingly.
    pub fn rda(&mut self) -> Result<(), TapeError> {
        for b in std::io::stdin().lock().bytes() {
            match b {
                Ok(byte) => {
                    if byte == 10 {
                        break;
                    }

                    self.store(byte)?;
                    self.step_fw()?;
                }
                Err(e) => return Err(TapeError::from(e)),
            }
        }

        Ok(())
    }

    /// Writes a null byte to the current cell and all following cells until a null byte is encountered.
    /// The pointer is incremented accordingly.
    pub fn clr(&mut self) -> Result<(), TapeError> {
        self.store(0)?;
        loop {
            self.step_fw()?;

            let b = self.get_current_value()?;

            if b == 0 {
                break;
            }

            self.store(0)?;
        }

        Ok(())
    }

    fn step_fw(&mut self) -> Result<(), TapeError> {
        let moved = self.ptr_index.checked_add(1);
        match moved {
            Some(n) => Ok(self.ptr_index = n),
            None => Err(TapeError::new(
                super::TapeErrorType::IndexError,
                "Moving the pointer 1 step forward would result in overshooting the tape."
                    .to_string(),
            )),
        }
    }

    fn store(&mut self, byte: u8) -> Result<(), TapeError> {
        self.data[self.ptr_index] = byte;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn tape_pfw_by_1_equals_current_plus_1_test() {
        let mut tape = super::Tape::default();

        tape.pfw(1).unwrap();

        assert_eq!(tape.ptr_index, 1);
    }

    #[test]
    #[should_panic]
    fn tape_pfw_by_200000_panics_test() {
        let mut tape = super::Tape::default();

        tape.pfw(200000).unwrap();
    }

    #[test]
    fn tape_pbw_by_n_equals_current_minus_n_test() {
        let mut tape = super::Tape::default();

        let fwd = 20;
        let bwd = 5;

        tape.pfw(fwd).unwrap();
        tape.pbw(bwd).unwrap();

        assert_eq!(tape.ptr_index, fwd - bwd);
    }

    #[test]
    fn tape_inc_by_n_equals_cell_value_plus_n_test() {
        let mut tape = super::Tape::default();

        let by = 72;

        let value_before = tape.data[tape.ptr_index];
        tape.inc(by).unwrap();
        let value_after = tape.data[tape.ptr_index];

        assert_eq!(value_after, value_before + by);
    }

    #[test]
    fn tape_dec_by_n_equals_cell_value_plus_n_test() {
        let mut tape = super::Tape::default();

        tape.inc(100).unwrap();
        let by = 50;
        tape.dec(by).unwrap();
        let value_after = tape.get_current_value();

        assert_eq!(value_after.unwrap(), 50);
    }

    #[test]
    #[should_panic]
    fn tape_dec_panics_test() {
        let mut tape = super::Tape::default();
        let by = 1;
        tape.dec(by).unwrap();
        let value_after = tape.get_current_value();

        assert_eq!(value_after.unwrap(), 50);
    }

    #[test]
    fn wrt_successful_test() {
        let mut tape = super::Tape::default();
        tape.inc(72).unwrap();
        let res = tape.wrt();

        assert!(!res.is_err());
    }

    #[test]
    fn wrti8_successful_test() {
        let mut tape = super::Tape::default();
        tape.inc(127).unwrap();
        let res = tape.wrti8();

        assert!(!res.is_err());
    }

    #[test]
    fn wrti16_successful_test() {
        let mut tape = super::Tape::default();
        tape.inc(16).unwrap();
        tape.pfw(1).unwrap();
        tape.inc(16).unwrap();
        tape.pbw(1).unwrap();
        let res = tape.wrti16();

        assert!(!res.is_err());
    }

    #[test]
    fn wrti32_successful_test() {
        let mut tape = super::Tape::default();
        tape.inc(16).unwrap();
        tape.pfw(1).unwrap();
        tape.inc(16).unwrap();
        tape.pfw(1).unwrap();
        tape.inc(16).unwrap();
        tape.pbw(2).unwrap();
        let res = tape.wrti32();

        assert!(!res.is_err());
    }

    #[test]
    fn wrti64_successful_test() {
        let mut tape = super::Tape::default();
        tape.inc(1).unwrap();
        tape.pfw(1).unwrap();
        tape.inc(0).unwrap();
        tape.pfw(1).unwrap();
        tape.inc(0).unwrap();
        tape.pfw(1).unwrap();
        tape.inc(0).unwrap();
        tape.pfw(1).unwrap();
        tape.inc(0).unwrap();
        tape.pfw(1).unwrap();
        tape.inc(0).unwrap();
        tape.pfw(1).unwrap();
        tape.inc(0).unwrap();
        tape.pfw(1).unwrap();
        tape.inc(0).unwrap();
        tape.pbw(7).unwrap();
        let res = tape.wrti64();

        assert!(!res.is_err());
    }

    #[test]
    fn wrtu8_successful_test() {
        let mut tape = super::Tape::default();
        tape.inc(255).unwrap();
        let res = tape.wrtu8();

        assert!(!res.is_err());
    }

    #[test]
    fn wrtu16_successful_test() {
        let mut tape = super::Tape::default();
        tape.inc(16).unwrap();
        tape.pfw(1).unwrap();
        tape.inc(16).unwrap();
        tape.pbw(1).unwrap();
        let res = tape.wrtu16();

        assert!(!res.is_err());
    }

    #[test]
    fn wrtu32_successful_test() {
        let mut tape = super::Tape::default();
        tape.inc(0).unwrap();
        tape.pfw(1).unwrap();
        tape.inc(0).unwrap();
        tape.pfw(1).unwrap();
        tape.inc(0).unwrap();
        tape.pfw(1).unwrap();
        tape.inc(1).unwrap();
        tape.pbw(3).unwrap();
        let res = tape.wrtu32();

        assert!(!res.is_err());
    }

    #[test]
    fn wrtu64_successful_test() {
        let mut tape = super::Tape::default();
        tape.inc(1).unwrap();
        tape.pfw(1).unwrap();
        tape.inc(0).unwrap();
        tape.pfw(1).unwrap();
        tape.inc(0).unwrap();
        tape.pfw(1).unwrap();
        tape.inc(0).unwrap();
        tape.pfw(1).unwrap();
        tape.inc(0).unwrap();
        tape.pfw(1).unwrap();
        tape.inc(0).unwrap();
        tape.pfw(1).unwrap();
        tape.inc(0).unwrap();
        tape.pfw(1).unwrap();
        tape.inc(0).unwrap();
        tape.pbw(7).unwrap();
        let res = tape.wrtu64();

        assert!(!res.is_err());
    }

    #[test]
    fn wrtf32_successful_test() {
        let mut tape = super::Tape::default();
        tape.inc(0).unwrap();
        tape.pfw(1).unwrap();
        tape.inc(0).unwrap();
        tape.pfw(1).unwrap();
        tape.inc(0).unwrap();
        tape.pfw(1).unwrap();
        tape.inc(1).unwrap();
        tape.pbw(3).unwrap();
        let res = tape.wrtf32();

        assert!(!res.is_err());
    }

    #[test]
    fn wrtf64_successful_test() {
        let mut tape = super::Tape::default();
        tape.inc(255).unwrap();
        tape.pfw(1).unwrap();
        tape.inc(0).unwrap();
        tape.pfw(1).unwrap();
        tape.inc(0).unwrap();
        tape.pfw(1).unwrap();
        tape.inc(0).unwrap();
        tape.pfw(1).unwrap();
        tape.inc(0).unwrap();
        tape.pfw(1).unwrap();
        tape.inc(0).unwrap();
        tape.pfw(1).unwrap();
        tape.inc(255).unwrap();
        tape.pfw(1).unwrap();
        tape.inc(255).unwrap();
        tape.pbw(7).unwrap();
        let res = tape.wrtf64();

        assert!(!res.is_err());
    }

    #[test]
    fn set_successful_test() {
        let mut tape = super::Tape::default();
        let res = tape.set("Hello");

        assert!(!res.is_err())
    }

    #[test]
    fn wra_successful_test() {
        let mut tape = super::Tape::default();

        let mut res = tape.set("Hello");
        assert!(!res.is_err());

        tape.pbw(5).unwrap();
        res = tape.wra();

        assert!(!res.is_err())
    }

    #[allow(dead_code)]
    fn rda_successful_test() {
        let mut tape = super::Tape::default();

        let res = tape.rda();

        assert!(!res.is_err())
    }

    #[test]
    fn clr_successful_test() {
        let mut tape = super::Tape::default();

        let res = tape.set("Hello");
        assert!(!res.is_err());

        tape.pbw(5).unwrap();
        tape.clr().unwrap();

        assert_eq!(tape.get_current_value().unwrap(), 0)
    }

    #[test]
    fn reset_successful_test() {
        let mut tape = super::Tape::default();

        tape.set("Hellooooooooooooooooooooooooooooooooooo!")
            .unwrap();

        tape.reset();

        assert_eq!(tape.get_current_value().unwrap(), 0);
    }

    #[allow(dead_code)]
    fn rdi_successful_test() {
        let mut tape = super::Tape::default();

        let rdi_res = tape.rdi();
        assert!(!rdi_res.is_err());

        let wrt_res = tape.wrt();
        assert!(!wrt_res.is_err());
    }
}

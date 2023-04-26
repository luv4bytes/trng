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
mod tape_num;

pub use error::{TapeError, TapeErrorType};

use std::io::{Read, Write};

use self::tape_num::TapeNum;

/// Type alias for a simple result with a TapeError.
pub type TapeResult<T> = Result<T, TapeError>;

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
    pub fn get_current_value(&mut self) -> TapeResult<u8> {
        let cur = self.data.get(self.ptr_index);

        match cur {
            Some(val) => Ok(*val),
            None => Err(TapeError::new(
                TapeErrorType::Index,
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
    pub fn pfw(&mut self, steps: usize) -> TapeResult<()> {
        if self.ptr_index + steps >= self.data.capacity() {
            return Err(TapeError::new(
                TapeErrorType::Index,
                format!(
                    "Moving the pointer {} step(s) forward would result in overshooting the tape.",
                    steps
                ),
            ));
        }

        self.ptr_index += steps;
        Ok(())
    }

    /// Moves the pointer (read/write head) backward.
    ///
    /// * `steps` - The number of steps to move backward on the tape.
    pub fn pbw(&mut self, steps: usize) -> TapeResult<()> {
        let subbed = self.ptr_index.checked_sub(steps);

        match subbed {
            Some(n) => {
                self.ptr_index = n;
                Ok(())
            }
            None => Err(TapeError::new(
                TapeErrorType::Index,
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
    pub fn inc(&mut self, by: u8) -> TapeResult<()> {
        let added = self.get_current_value()?.checked_add(by);

        match added {
            Some(n) => {
                self.data[self.ptr_index] = n;
                Ok(())
            }
            None => Err(TapeError::new(
                TapeErrorType::Overflow,
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
    pub fn dec(&mut self, by: u8) -> TapeResult<()> {
        let subbed = self.get_current_value()?.checked_sub(by);

        match subbed {
            Some(n) => {
                self.data[self.ptr_index] = n;
                Ok(())
            }
            None => Err(TapeError::new(
                TapeErrorType::Overflow,
                format!(
                    "Subtracting {} from the current cell value would result in an overflow.",
                    by
                ),
            )),
        }
    }

    /// Writes the value of the current cell to stdout.
    pub fn wrt(&mut self) -> TapeResult<()> {
        if let Err(e) = std::io::stdout()
            .lock()
            .write_all(&[self.get_current_value()?])
        {
            return Err(TapeError::from(e));
        }

        Ok(())
    }

    /// Writes the value of the current cell as an 8-bit signed integer to stdout.
    pub fn wrti8(&mut self) -> TapeResult<()> {
        self.wrt_tape_num::<i8>()?;

        Ok(())
    }

    /// Writes the current cell and the next interpreted as an 16-bit signed integer to stdout.
    pub fn wrti16(&mut self) -> TapeResult<()> {
        self.wrt_tape_num::<i16>()?;

        Ok(())
    }

    /// Writes the current cell and the next three interpreted as an 32-bit signed integer to stdout.
    pub fn wrti32(&mut self) -> TapeResult<()> {
        self.wrt_tape_num::<i32>()?;

        Ok(())
    }

    /// Writes the current cell and the next seven interpreted as an 64-bit signed integer to stdout.
    pub fn wrti64(&mut self) -> TapeResult<()> {
        self.wrt_tape_num::<i64>()?;

        Ok(())
    }

    /// Writes the value of the current cell as an 8-bit unsigned integer to stdout.
    pub fn wrtu8(&mut self) -> TapeResult<()> {
        self.wrt_tape_num::<u8>()?;

        Ok(())
    }

    /// Writes the current cell and the next interpreted as an 16-bit unsigned integer to stdout.
    pub fn wrtu16(&mut self) -> TapeResult<()> {
        self.wrt_tape_num::<u16>()?;

        Ok(())
    }

    /// Writes the current cell and the next three interpreted as an 32-bit signed integer to stdout.
    pub fn wrtu32(&mut self) -> TapeResult<()> {
        self.wrt_tape_num::<u32>()?;

        Ok(())
    }

    /// Writes the current cell and the next seven interpreted as an 64-bit unsigned integer to stdout.
    pub fn wrtu64(&mut self) -> TapeResult<()> {
        self.wrt_tape_num::<u64>()?;

        Ok(())
    }

    /// Writes the current cell and the next three interpreted as an 32-bit floating point number to stdout.
    pub fn wrtf32(&mut self) -> TapeResult<()> {
        self.wrt_tape_num::<f32>()?;

        Ok(())
    }

    /// Writes the current cell and the next seven interpreted as an 64-bit floating point number to stdout.
    pub fn wrtf64(&mut self) -> TapeResult<()> {
        self.wrt_tape_num::<f64>()?;

        Ok(())
    }

    /// Reads a character from stdin and stores it in the current cell.
    pub fn rdi(&mut self) -> TapeResult<()> {
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
    pub fn set(&mut self, v: &str) -> TapeResult<()> {
        for b in v.as_bytes() {
            self.store(*b)?;
            self.step_fw()?;
        }

        Ok(())
    }

    /// Sets the given value as an 8-bit signed integer.
    /// * `v` - The value to set.
    pub fn seti8(&mut self, v: i8) -> TapeResult<()> {
        self.set_tape_num(v)?;

        Ok(())
    }

    /// Sets the given value as an 16-bit signed integer.
    /// * `v` - The value to set.
    pub fn seti16(&mut self, v: i16) -> TapeResult<()> {
        self.set_tape_num(v)?;

        Ok(())
    }

    /// Sets the given value as an 32-bit signed integer.
    /// * `v` - The value to set.
    pub fn seti32(&mut self, v: i32) -> TapeResult<()> {
        self.set_tape_num(v)?;

        Ok(())
    }

    /// Sets the given value as an 64-bit signed integer.
    /// * `v` - The value to set.
    pub fn seti64(&mut self, v: i64) -> TapeResult<()> {
        self.set_tape_num(v)?;

        Ok(())
    }

    /// Sets the given value as an 8-bit unsigned integer.
    /// * `v` - The value to set.
    pub fn setu8(&mut self, v: u8) -> TapeResult<()> {
        self.set_tape_num(v)?;

        Ok(())
    }

    /// Sets the given value as an 16-bit unsigned integer.
    /// * `v` - The value to set.
    pub fn setu16(&mut self, v: u16) -> TapeResult<()> {
        self.set_tape_num(v)?;

        Ok(())
    }

    /// Sets the given value as an 32-bit unsigned integer.
    /// * `v` - The value to set.
    pub fn setu32(&mut self, v: u32) -> TapeResult<()> {
        self.set_tape_num(v)?;

        Ok(())
    }

    /// Sets the given value as an 64-bit unsigned integer.
    /// * `v` - The value to set.
    pub fn setu64(&mut self, v: u64) -> TapeResult<()> {
        self.set_tape_num(v)?;

        Ok(())
    }

    /// Sets the given value as a 32-bit float.
    /// * `v` - The value to set.
    pub fn setf32(&mut self, v: f32) -> TapeResult<()> {
        self.set_tape_num(v)?;

        Ok(())
    }

    /// Sets the given value as a 64-bit float.
    /// * `v` - The value to set.
    pub fn setf64(&mut self, v: f64) -> TapeResult<()> {
        self.set_tape_num(v)?;

        Ok(())
    }

    /// Writes the current cell and all following cells to stdout until a null byte is encountered.
    /// The pointer is incremented accordingly.
    pub fn wra(&mut self) -> TapeResult<()> {
        loop {
            let b = self.get_current_value()?;

            if b == 0 {
                break;
            }

            match std::io::stdout().lock().write_all(&[b]) {
                Ok(_) => self.step_fw()?,
                Err(e) => return Err(TapeError::from(e)),
            }
        }

        Ok(())
    }

    /// Reads all bytes from stdin until LF is encountered.
    /// The pointer is incremented accordingly.
    pub fn rda(&mut self) -> TapeResult<()> {
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
    pub fn clr(&mut self) -> TapeResult<()> {
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

    fn step_fw(&mut self) -> TapeResult<()> {
        let moved = self.ptr_index.checked_add(1);
        match moved {
            Some(n) => {
                self.ptr_index = n;
                Ok(())
            }
            None => Err(TapeError::new(
                TapeErrorType::Index,
                "Moving the pointer 1 step forward would result in overshooting the tape."
                    .to_string(),
            )),
        }
    }

    fn store(&mut self, byte: u8) -> TapeResult<()> {
        self.data[self.ptr_index] = byte;
        Ok(())
    }

    fn set_tape_num<T: TapeNum>(&mut self, v: T) -> TapeResult<()> {
        for byte in v.get_bytes() {
            self.store(byte)?;
            self.step_fw()?;
        }

        Ok(())
    }

    fn wrt_tape_num<T: TapeNum>(&mut self) -> TapeResult<()> {
        let slice = &self.data[self.ptr_index..self.ptr_index + T::number_of_bytes()];
        let v = slice.to_vec();

        let tv = T::from(v).to_string();

        if let Err(e) = std::io::stdout().lock().write_all(tv.as_bytes()) {
            return Err(TapeError::from(e));
        }

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
    fn seti8_successful_test() {
        let mut tape = super::Tape::default();
        let res = tape.seti8(127);

        assert!(!res.is_err())
    }

    #[test]
    fn seti16_successful_test() {
        let mut tape = super::Tape::default();
        let res = tape.seti16(1031);

        assert!(!res.is_err())
    }

    #[test]
    fn seti32_successful_test() {
        let mut tape = super::Tape::default();
        let res = tape.seti32(i32::MAX);

        assert!(!res.is_err())
    }

    #[test]
    fn seti64_successful_test() {
        let mut tape = super::Tape::default();
        let res = tape.seti64(i64::MAX);

        assert!(!res.is_err())
    }

    #[test]
    fn setu8_successful_test() {
        let mut tape = super::Tape::default();
        let res = tape.setu8(127);

        assert!(!res.is_err())
    }

    #[test]
    fn setu16_successful_test() {
        let mut tape = super::Tape::default();
        let res = tape.setu16(1031);

        assert!(!res.is_err())
    }

    #[test]
    fn setu32_successful_test() {
        let mut tape = super::Tape::default();
        let res = tape.setu32(u32::MAX);

        assert!(!res.is_err())
    }

    #[test]
    fn setu64_successful_test() {
        let mut tape = super::Tape::default();
        let res = tape.setu64(u64::MAX);

        assert!(!res.is_err())
    }

    #[test]
    fn setf32_successful_test() {
        let mut tape = super::Tape::default();
        let res = tape.setf32(f32::MAX);

        assert!(!res.is_err())
    }

    #[test]
    fn setf64_successful_test() {
        let mut tape = super::Tape::default();
        let res = tape.setf64(f64::MAX);

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

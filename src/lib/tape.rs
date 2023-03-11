use std::io::{Read, Write};

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
    pub fn new(data_band_sz: usize) -> Tape {
        Tape {
            data: vec![0; data_band_sz],
            ptr_index: 0,
        }
    }

    /// Gets the value of the current cell.
    pub fn get_current_value(&mut self) -> u8 {
        self.data[self.ptr_index]
    }

    /// Moves the pointer (read/write head) forward.
    ///
    /// * `steps` - The number of steps to move forward on the tape.
    pub fn pfw(&mut self, steps: usize) -> () {
        self.ptr_index = self.ptr_index + steps;
    }

    /// Moves the pointer (read/write head) backward.
    ///
    /// * `steps` - The number of steps to move backward on the tape.
    pub fn pbw(&mut self, steps: usize) -> () {
        self.ptr_index = self.ptr_index - steps;
    }

    /// Increments the value of the current cell
    ///
    /// * `by` - This value gets added to the value of the current cell.
    pub fn inc(&mut self, by: u8) -> () {
        self.data[self.ptr_index] = self.get_current_value() + by;
    }

    /// Decrements the value of the current cell
    ///
    /// * `by` - This value gets subtracted from the value of the current cell.
    pub fn dec(&mut self, by: u8) -> () {
        self.data[self.ptr_index] = self.get_current_value() - by;
    }

    /// Writes the value of the current cell to stdout.
    pub fn wrt(&mut self) -> Result<(), std::io::Error> {
        std::io::stdout()
            .lock()
            .write_all(&[self.get_current_value()])?;
        Ok(())
    }

    /// Writes the value of the current cell as an integer to stdout.
    pub fn wrti(&mut self) -> Result<(), std::io::Error> {
        let b = self.get_current_value();
        let i = b.to_string();

        std::io::stdout().lock().write_all(i.as_bytes())?;

        Ok(())
    }

    /// Reads a character from stdin and stores it in the current cell.
    pub fn rdi(&mut self) -> Result<(), std::io::Error> {
        let b = std::io::stdin().lock().bytes().next();
        match b {
            Some(res) => match res {
                Ok(byte) => Ok(self.store(byte)),
                Err(e) => Err(e),
            },
            None => Ok(()),
        }
    }

    /// Sets the given value, placing each byte in a separate cell and incrementing the pointer accordingly.
    /// * `v` - The value to set.
    pub fn set(&mut self, v: &str) -> Result<(), std::io::Error> {
        for b in v.as_bytes() {
            self.data[self.ptr_index] = *b;
            self.ptr_index += 1;
        }

        Ok(())
    }

    /// Writes the current cell and all following cells to stdout until a null byte is encountered.
    /// The pointer is incremented accordingly.
    pub fn wra(&mut self) -> Result<(), std::io::Error> {
        loop {
            let b = self.get_current_value();

            if b == 0 {
                break;
            }

            match std::io::stdout().lock().write_all(&[b]) {
                Ok(_) => {
                    self.step_fw();
                }
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }

    /// Reads all bytes from stdin until LF is encountered.
    /// The pointer is incremented accordingly.
    pub fn rda(&mut self) -> Result<(), std::io::Error> {
        for b in std::io::stdin().lock().bytes() {
            match b {
                Ok(byte) => {
                    if byte == 10 {
                        break;
                    }

                    self.store(byte);
                    self.step_fw();
                }
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }

    /// Writes a null byte to the current cell and all following cells until a null byte is encountered.
    /// The pointer is incremented accordingly.
    pub fn clr(&mut self) {
        self.store(0);
        loop {
            self.step_fw();

            let b = self.get_current_value();

            if b == 0 {
                break;
            }

            self.store(0);
        }
    }

    fn step_fw(&mut self) {
        self.ptr_index += 1;
    }

    fn store(&mut self, byte: u8) -> () {
        self.data[self.ptr_index] = byte;
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn tape_pfw_by_1_equals_current_plus_1_test() {
        let mut tape = super::Tape::default();

        tape.pfw(1);

        assert_eq!(tape.ptr_index, 1);
    }

    #[test]
    fn tape_pbw_by_n_equals_current_minus_n_test() {
        let mut tape = super::Tape::default();

        let fwd = 20;
        let bwd = 5;

        tape.pfw(fwd);
        tape.pbw(bwd);

        assert_eq!(tape.ptr_index, fwd - bwd);
    }

    #[test]
    fn tape_inc_by_n_equals_cell_value_plus_n_test() {
        let mut tape = super::Tape::default();

        let by = 72;

        let value_before = tape.data[tape.ptr_index];
        tape.inc(by);
        let value_after = tape.data[tape.ptr_index];

        assert_eq!(value_after, value_before + by);
    }

    #[test]
    fn tape_dec_by_n_equals_cell_value_plus_n_test() {
        let mut tape = super::Tape::default();

        tape.inc(100);
        let by = 50;
        tape.dec(by);
        let value_after = tape.get_current_value();

        assert_eq!(value_after, 50);
    }

    #[test]
    fn wrt_successful_test() {
        let mut tape = super::Tape::default();
        tape.inc(72);
        let res = tape.wrt();

        assert!(!res.is_err());
    }

    #[test]
    fn wrti_successful_test() {
        let mut tape = super::Tape::default();
        tape.inc(255);
        let res = tape.wrti();

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

        tape.pbw(5);
        res = tape.wra();

        assert!(!res.is_err())
    }

    #[test]
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

        tape.pbw(5);
        tape.clr();

        assert_eq!(tape.get_current_value(), 0)
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

const DEFAULT_TAPE_SIZE: usize = 30000;

pub struct InterpreterSettings {
    pub data_tape_sz: usize,
}

impl InterpreterSettings {
    pub fn new(data_tape_sz: usize) -> InterpreterSettings {
        InterpreterSettings {
            data_tape_sz: data_tape_sz,
        }
    }
}

impl Default for InterpreterSettings {
    fn default() -> Self {
        Self {
            data_tape_sz: DEFAULT_TAPE_SIZE,
        }
    }
}

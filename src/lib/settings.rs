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

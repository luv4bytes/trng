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

use std::fmt::Display;

/// Defines functions for numeric types used in the tape.
pub trait Num: Display {
    /// Returns a vector with the bytes of the current instance in big-endian byte order.
    fn get_bytes(self) -> Vec<u8>;

    /// Returns the number of bytes of the type.
    fn number_of_bytes() -> usize;

    /// Creates an instance from a vector of bytes.
    fn from(vec: Vec<u8>) -> Self;
}

impl Num for u8 {
    fn get_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn number_of_bytes() -> usize {
        1
    }

    fn from(vec: Vec<u8>) -> Self {
        let mut b: [u8; 1] = [0];
        b[0] = vec[0];
        u8::from_be_bytes(b)
    }
}

impl Num for i8 {
    fn get_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn number_of_bytes() -> usize {
        1
    }

    fn from(vec: Vec<u8>) -> Self {
        let mut b: [u8; 1] = [0];
        b[0] = vec[0];
        i8::from_be_bytes(b)
    }
}

impl Num for u16 {
    fn get_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn number_of_bytes() -> usize {
        2
    }

    fn from(vec: Vec<u8>) -> Self {
        let mut b: [u8; 2] = [0, 0];
        b[0] = vec[0];
        b[1] = vec[1];
        u16::from_be_bytes(b)
    }
}

impl Num for i16 {
    fn get_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn number_of_bytes() -> usize {
        2
    }

    fn from(vec: Vec<u8>) -> Self {
        let mut b: [u8; 2] = [0, 0];
        b[0] = vec[0];
        b[1] = vec[1];
        i16::from_be_bytes(b)
    }
}

impl Num for u32 {
    fn get_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn number_of_bytes() -> usize {
        4
    }

    fn from(vec: Vec<u8>) -> Self {
        let mut b: [u8; 4] = [0, 0, 0, 0];
        b[0] = vec[0];
        b[1] = vec[1];
        b[2] = vec[2];
        b[3] = vec[3];
        u32::from_be_bytes(b)
    }
}

impl Num for i32 {
    fn get_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn number_of_bytes() -> usize {
        4
    }

    fn from(vec: Vec<u8>) -> Self {
        let mut b: [u8; 4] = [0, 0, 0, 0];
        b[0] = vec[0];
        b[1] = vec[1];
        b[2] = vec[2];
        b[3] = vec[3];
        i32::from_be_bytes(b)
    }
}

impl Num for u64 {
    fn get_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn number_of_bytes() -> usize {
        8
    }

    fn from(vec: Vec<u8>) -> Self {
        let mut b: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
        b[0] = vec[0];
        b[1] = vec[1];
        b[2] = vec[2];
        b[3] = vec[3];
        b[4] = vec[4];
        b[5] = vec[5];
        b[6] = vec[6];
        b[7] = vec[7];
        u64::from_be_bytes(b)
    }
}

impl Num for i64 {
    fn get_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn number_of_bytes() -> usize {
        8
    }

    fn from(vec: Vec<u8>) -> Self {
        let mut b: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
        b[0] = vec[0];
        b[1] = vec[1];
        b[2] = vec[2];
        b[3] = vec[3];
        b[4] = vec[4];
        b[5] = vec[5];
        b[6] = vec[6];
        b[7] = vec[7];
        i64::from_be_bytes(b)
    }
}

impl Num for f32 {
    fn get_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn number_of_bytes() -> usize {
        4
    }

    fn from(vec: Vec<u8>) -> Self {
        let mut b: [u8; 4] = [0, 0, 0, 0];
        b[0] = vec[0];
        b[1] = vec[1];
        b[2] = vec[2];
        b[3] = vec[3];
        f32::from_be_bytes(b)
    }
}

impl Num for f64 {
    fn get_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn number_of_bytes() -> usize {
        8
    }

    fn from(vec: Vec<u8>) -> Self {
        let mut b: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
        b[0] = vec[0];
        b[1] = vec[1];
        b[2] = vec[2];
        b[3] = vec[3];
        b[4] = vec[4];
        b[5] = vec[5];
        b[6] = vec[6];
        b[7] = vec[7];
        f64::from_be_bytes(b)
    }
}

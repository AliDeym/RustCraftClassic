/*
    Copyright (c) 2020 Ali Deym

    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.
*/

pub struct Vec3D<T=u16> (pub T, pub T, pub T) where T: Copy, T: Sized;

impl<T> Vec3D<T> where T: Copy, T: Sized {
    pub fn new(x: T, y: T, z: T) -> Vec3D<T> {
        Vec3D { 0: x, 1: y, 2: z }
    }

    pub fn get_x(&self) -> T {
        self.0
    }

    pub fn get_y(&self) -> T {
        self.1
    }

    pub fn get_z(&self) -> T {
        self.2
    }

    pub fn set_x(&mut self, x: T) {
        self.0 = x;
    }

    pub fn set_y(&mut self, y: T) {
        self.1 = y;
    }

    pub fn set_z(&mut self, z: T) {
        self.2 = z;
    }
}

pub struct BufferWriter {
    buffer: Vec<u8>,
}

impl BufferWriter {
    pub fn new(size: usize) -> BufferWriter {
        BufferWriter {
            buffer: Vec::<u8>::with_capacity(size),
        }
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.buffer
    }

    pub fn write_byte(&mut self, data: u8) {
        self.buffer.push(data);
    }

    pub fn write_sbyte(&mut self, data: i8) {
        self.buffer.push(data as u8);
    }

    pub fn write_short(&mut self, data: u16) {
        let bytes = data.to_be_bytes();

        self.buffer.extend(&bytes[..]);
    }

    pub fn write_uint(&mut self, data: u32) {
        let bytes = data.to_be_bytes();

        self.buffer.extend(&bytes[..]);
    }

    pub fn write_int(&mut self, data: i32) {
        let bytes = data.to_be_bytes();

        self.buffer.extend(&bytes[..]);
    }

    pub fn write_string(&mut self, data: &str) {
        let mut char_iter = data.as_bytes().iter();

        for _ in 0..64 {
            self.buffer.push(*char_iter.next().unwrap_or(&b' ') as u8);
        }
    }

    pub fn write_array(&mut self, data: &[u8]) {
        if data.len() > 1024 {
            self.buffer.extend(data.split_at(1024).0);

            return;
        }

        self.buffer.extend(data);
    }
}

pub struct BufferReader<'a> {
    index: usize,
    buffer: &'a Vec<u8>,
}

impl<'a> BufferReader<'a> {
    pub fn new(buffer: &'a Vec<u8>) -> BufferReader {
        BufferReader { index: 0, buffer }
    }

    pub fn read_byte(&mut self) -> u8 {
        self.index += 1;

        *self.buffer.get(self.index - 1).unwrap_or(&0)
    }

    pub fn read_sbyte(&mut self) -> i8 {
        self.index += 1;

        *self.buffer.get(self.index - 1).unwrap_or(&0) as i8
    }

    pub fn read_ushort(&mut self) -> u16 {
        self.index += 2;

        let b1 = self.read_byte();
        let b2 = self.read_byte();

        (b1 as u16) << 8 | b2 as u16
    }

    pub fn read_string(&mut self) -> String {
        let grabbed = String::from_utf8(self.buffer[self.index..self.index + 64].to_vec())
            .unwrap_or(String::from(""));

        self.index += 64;

        String::from(grabbed.trim())
    }
}

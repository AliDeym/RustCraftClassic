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

pub fn math_min(num1: u8, num2: u8) -> u8 {
    if num1 < num2 {
        num1
    } else {
        num2
    }
}

pub struct Vec3D<T = u16>(pub T, pub T, pub T)
where
    T: Copy,
    T: Sized,
    T: Clone;

impl Clone for Vec3D {
    fn clone(&self) -> Self {
        Self(self.0, self.1, self.2)
    }
}
impl Copy for Vec3D {}

impl<T> Vec3D<T>
where
    T: Copy,
    T: Sized,
    T: Clone,
{
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

pub struct Transform {
    position: Vec3D,
    yaw: u8,
    pitch: u8,
}

impl Clone for Transform {
    fn clone(&self) -> Self {
        Self::new(self.position, self.yaw, self.pitch)
    }
}

impl Transform {
    pub fn new(position: Vec3D, yaw: u8, pitch: u8) -> Transform {
        Transform {
            position,
            yaw,
            pitch,
        }
    }

    pub fn default() -> Transform {
        Self::new(Vec3D::new(0, 0, 0), 0, 0)
    }

    pub fn get_pos(&self) -> &Vec3D {
        &self.position
    }

    pub fn get_yaw(&self) -> u8 {
        self.yaw
    }

    pub fn get_pitch(&self) -> u8 {
        self.pitch
    }

    pub fn set_pos(&mut self, x: u16, y: u16, z: u16) {
        self.position.0 = x;
        self.position.1 = y;
        self.position.2 = z;
    }

    pub fn set_yaw(&mut self, yaw: u8) {
        self.yaw = yaw;
    }

    pub fn set_pitch(&mut self, pitch: u8) {
        self.pitch = pitch;
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

    pub fn write_vec3d(&mut self, data: &Vec3D) {
        self.write_short(data.0);
        self.write_short(data.1);
        self.write_short(data.2);
    }

    pub fn write_transform(&mut self, data: &Transform) {
        self.write_vec3d(data.get_pos());

        self.write_byte(data.yaw);
        self.write_byte(data.pitch);
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

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn read_byte(&mut self) -> u8 {
        self.index += 1;

        // In case we receive a timeout/packetloss, 0xff (255) will be returned.
        *self.buffer.get(self.index - 1).unwrap_or(&0xff)
    }

    pub fn read_sbyte(&mut self) -> i8 {
        self.index += 1;

        // In case we receive a timeout/packetloss, 0xff (255) will be returned.
        *self.buffer.get(self.index - 1).unwrap_or(&0xff) as i8
    }

    pub fn read_ushort(&mut self) -> u16 {
        let b1 = self.read_byte();
        let b2 = self.read_byte();

        (b1 as u16) << 8 | b2 as u16
    }

    pub fn read_ushort_le(&mut self) -> u16 {
        let b1 = self.read_byte();
        let b2 = self.read_byte();

        (b2 as u16) << 8 | b1 as u16
    }

    pub fn read_short(&mut self) -> i16 {
        let b1 = self.read_byte();
        let b2 = self.read_byte();

        (b1 as i16) << 8 | b2 as i16
    }

    pub fn read_short_le(&mut self) -> i16 {
        let b1 = self.read_byte();
        let b2 = self.read_byte();

        (b2 as i16) << 8 | b1 as i16
    }

    pub fn read_string(&mut self) -> String {
        let grabbed = String::from_utf8(self.buffer[self.index..self.index + 64].to_vec())
            .unwrap_or(String::from(""));

        self.index += 64;

        String::from(grabbed.trim())
    }

    /// Reads to the end of the stream.
    /// NOTE that this method will not change the buffer index.
    pub fn read_to_end(&mut self) -> Vec<u8> {
        self.buffer[self.index..self.buffer.len() - self.index + 1].to_vec()
    }
}

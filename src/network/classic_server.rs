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

use super::super::core::BufferWriter;
use super::NetworkPacket;

pub struct ServerIdentification {
    protocol_version: u8,
    servername: String,
    motd: String,
    user_type: u8,
}

impl ServerIdentification {
    pub const ID: u8 = 0x00;
    pub const SIZE: usize = 131;

    pub fn new(
        protocol_version: u8,
        servername: String,
        motd: String,
        user_type: u8,
    ) -> ServerIdentification {
        ServerIdentification {
            protocol_version,
            servername,
            motd,
            user_type,
        }
    }
}

impl NetworkPacket for ServerIdentification {
    fn get_id(&self) -> u8 {
        Self::ID
    }
    fn get_size(&self) -> usize {
        Self::SIZE
    }

    fn handle_send(&self, buffer: &mut BufferWriter) {
        buffer.write_byte(self.protocol_version);

        buffer.write_string(&self.servername);
        buffer.write_string(&self.motd);

        buffer.write_byte(self.user_type);
    }
}

pub struct LevelInitialize;

impl LevelInitialize {
    pub const ID: u8 = 0x02;
    pub const SIZE: usize = 1;

    pub fn new() -> LevelInitialize {
        LevelInitialize
    }
}

impl NetworkPacket for LevelInitialize {
    fn get_id(&self) -> u8 {
        Self::ID
    }
    fn get_size(&self) -> usize {
        Self::SIZE
    }
}

pub struct LevelDataChunk {
    chunk_length: u16,
    chunk_data: Vec<u8>,
    percent_complete: u8,
}

impl LevelDataChunk {
    pub const ID: u8 = 0x03;
    pub const SIZE: usize = 1028;

    pub fn new(chunk_length: u16, chunk_data: Vec<u8>, percent_complete: u8) -> LevelDataChunk {
        LevelDataChunk {
            chunk_length,
            chunk_data,
            percent_complete,
        }
    }
}

impl NetworkPacket for LevelDataChunk {
    fn get_id(&self) -> u8 {
        Self::ID
    }
    fn get_size(&self) -> usize {
        Self::SIZE
    }

    fn handle_send(&self, buffer: &mut BufferWriter) {
        buffer.write_short(self.chunk_length);

        buffer.write_array(&self.chunk_data);

        buffer.write_byte(self.percent_complete);
    }
}
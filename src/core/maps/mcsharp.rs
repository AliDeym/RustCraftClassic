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
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use super::super::{util::BufferReader, Core, Map, Vec3D};
use super::MemoryMap;

use flate2::read::GzDecoder;

// Uses an internal memory map which is generated by the given file.
pub struct MCSharpMap {
    size: Vec3D,

    spawn_yaw: u8,
    spawn_pitch: u8,
    spawn_point: Vec3D,

    internal_map: MemoryMap,
}

impl MCSharpMap {
    pub fn try_new(file: &str) -> Option<MCSharpMap> {
        let file_name = format!("maps/{}.lvl", file);

        // Check if file exists.
        if !Path::new(&file_name).exists() {
            Core::static_log(&format!(
                "Path does not exist for loading the map \"{}\".",
                &file
            ));

            return None;
        }

        let f = File::open(&file_name);
        // Unable to read file.
        if !f.is_ok() {
            Core::static_log(&format!("Unable to read map file \"{}\".", &file_name));

            return None;
        }

        let mut f = f.unwrap();
        let mut f_buffer = Vec::new();

        // Read the file.
        let f_result = f.read_to_end(&mut f_buffer);

        if !f_result.is_ok() {
            Core::static_log(&format!("Failed to read the whole map file \"{}\".", &file));

            return None;
        }

        // Wrap the data inside a GzDecoder.
        let mut gz_stream = GzDecoder::new(&f_buffer[..]);
        let mut data = Vec::new();

        gz_stream.read_to_end(&mut data).unwrap();

        let mut reader = BufferReader::new(&data);

        /* HEADER */
        // Read magic number.
        let magic_num = reader.read_ushort_le();

        if magic_num != 0x752 {
            Core::static_log(&format!(
                "Magic number mismatch for loading map \"{}\".",
                &file
            ));
            Core::static_log(&format!("Found: {:#08x}", magic_num));

            return None;
        }

        // Map has to be read in Int16, but our app works with UInt16.
        let size = Vec3D::new(
            reader.read_short_le() as u16,
            reader.read_short_le() as u16,
            reader.read_short_le() as u16,
        );
        let Vec3D(w, d, h) = size;

        let mut internal_map = MemoryMap::new(size);

        let spawn_x = reader.read_short_le() * 32;
        let spawn_y = reader.read_short_le() * 32;
        let spawn_z = reader.read_short_le() * 32;

        // Next 3 integers are spawn area.
        //let spawn_point = Vec3D::new(reader.read_short_le() * 32, reader.read_short_le() * 32, reader.read_short_le() * 32);
        let spawn_point = Vec3D(spawn_x as u16, spawn_y as u16, spawn_z as u16);

        // And two bytes for yaw and pitch.
        let yaw = reader.read_byte();
        let pitch = reader.read_byte();

        // Visit permission, and build permission (Not needed).
        reader.read_byte();
        reader.read_byte();

        /* DATA CHUNK */
        let data = reader.read_to_end();

        internal_map.set_data_chunks(data);

        let returning_map = MCSharpMap {
            size,

            spawn_point,

            spawn_yaw: yaw,
            spawn_pitch: pitch,

            internal_map,
        };

        Some(returning_map)
    }
}

impl Map for MCSharpMap {
    fn get_size(&self) -> &Vec3D {
        &self.size
    }

    fn get_chunks(&self) -> &Vec<u8> {
        &self.internal_map.get_chunks()
    }

    fn get_block(&self, position: &Vec3D) -> u8 {
        self.internal_map.get_block(position)
    }

    fn set_block(&mut self, position: &Vec3D, block: u8) {
        self.internal_map.set_block(position, block);
    }

    fn get_spawnarea(&self) -> Vec3D {
        self.spawn_point
    }

    fn get_spawnpitch(&self) -> u8 {
        self.spawn_pitch
    }

    fn get_spawnyaw(&self) -> u8 {
        self.spawn_yaw
    }
}

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

use super::Vec3D;

pub trait Map {
    fn get_magic_id(&self) -> i32 { 0x271bb788 }
    fn get_version_number(&self) -> u8 { 2 }

    fn get_size(&self) -> &Vec3D;
    
    fn get_chunks(&self) -> &Vec<u8>;


    fn get_spawnarea(&self) -> Vec3D {
        let size = self.get_size();

        Vec3D::new(size.get_x() * 16, size.get_y() * 32, size.get_z() * 16) 
    }

    fn get_block(&self, position: &Vec3D) -> u8;
    fn set_block(&mut self, position: &Vec3D, block: u8);
}

pub struct MemoryMap {
    size: Vec3D,
    data: Vec<u8>
}

impl MemoryMap {
    pub fn new(size: Vec3D) -> MemoryMap {
        let Vec3D (w, d, h) = size;

        let map = vec![0x0; w as usize * d as usize * h as usize];
        // Testing a map with a layer of grass.
        // Note that currently using set_block only is very costly for CPU.
        // A function is needed to change or set multiple blocks quickly and efficiently.
        let mut returning_map = MemoryMap {
            data: map,
            size,
        };
        
        for x in 0..w {
            for y in 0..h {
                returning_map.set_block(&Vec3D::new(x, d / 2 - 1, y), 2);
            }
        }

        returning_map
    }

    pub fn get_data_index(&self, position: &Vec3D) -> usize {
        let Vec3D (width, _depth, height) = self.get_size();

        (position.get_x() + (position.get_z() * width) + (position.get_y() * width * height)) as usize
    }
}

impl Map for MemoryMap {
    fn get_size(&self) -> &Vec3D {
        &self.size
    }
    
    fn get_chunks(&self) -> &Vec<u8> {
        &self.data
    }

    fn get_block(&self, position: &Vec3D) -> u8 {
        self.data[self.get_data_index(position)]
    }

    fn set_block(&mut self, position: &Vec3D, block: u8) {
        let index = self.get_data_index(position);

        self.data[index] = block;
    }
}
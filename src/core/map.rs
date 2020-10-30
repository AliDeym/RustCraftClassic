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
    fn get_magic_id(&self) -> i32 {
        0x271bb788
    }
    fn get_version_number(&self) -> u8 {
        2
    }

    fn get_size(&self) -> &Vec3D;

    fn get_chunks(&self) -> &Vec<u8>;

    fn get_spawnarea(&self) -> Vec3D {
        let size = self.get_size();

        Vec3D::new(size.get_x() * 16, size.get_y() * 32, size.get_z() * 16)
    }

    fn get_spawnyaw(&self) -> u8 {
        0
    }

    fn get_spawnpitch(&self) -> u8 {
        255
    }

    fn get_block(&self, position: &Vec3D) -> u8;
    fn set_block(&mut self, position: &Vec3D, block: u8);
}

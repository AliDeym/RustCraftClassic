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

use super::{Map, Vec3D};

pub struct World {
    name: String,
    players: Vec<usize>,
    map: Box<dyn Map>,
}

impl World {
    pub fn new(name: String, map: Box<dyn Map>) -> World {
        World {
            name,
            players: vec![],
            map,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn add_player(&mut self, player_uid: usize) {
        self.players.push(player_uid);
    }

    pub fn remove_player(&mut self, player_uid: usize) {
        let mut deleting_index = 0;
        // TODO: use better functions.
        for (index, player) in self.players.iter().enumerate() {
            if *player == player_uid {
                deleting_index = index;
            }
        }

        if deleting_index <= 0 {
            return;
        }

        self.players.remove(deleting_index);
    }

    pub fn get_players(&self) -> &Vec<usize> {
        &self.players
    }

    pub fn set_block(&mut self, coordinates: &Vec3D, mut block: u8, destroy: bool) {
        // TODO: Handle block deleting properly.
        // TODO: Change mutable block argument to immutable.
        if destroy {
            block = 0x0;
        }

        self.map.set_block(coordinates, block);
    }

    pub fn get_block(&self, coordinates: &Vec3D) -> u8 {
        self.map.get_block(coordinates)
    }

    pub fn get_chunks(&self) -> &Vec<u8> {
        self.map.get_chunks()
    }

    pub fn get_size(&self) -> &Vec3D {
        self.map.get_size()
    }

    pub fn get_spawnarea(&self) -> Vec3D {
        self.map.get_spawnarea()
    }

    pub fn get_spawnyaw(&self) -> u8 {
        self.map.get_spawnyaw()
    }

    pub fn get_spawnpitch(&self) -> u8 {
        self.map.get_spawnpitch()
    }

    // TODO: Add unload function to safely unload and save the map.
    // TODO: Add save functionality, both to world and to map.
}

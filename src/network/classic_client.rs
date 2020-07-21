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

use super::super::core::{BufferReader, Core, Vec3D};
use super::*;

pub struct PlayerIdentification {
    sender: usize,
    protocol_version: u8,
    username: String,
    verification_key: String,
    magic_number: u8,
}

impl PlayerIdentification {
    pub const ID: u8 = 0x00;
    pub const SIZE: usize = 131;

    pub fn new(buffer_reader: &mut BufferReader, sender: usize) -> PlayerIdentification {
        let protocol_version = buffer_reader.read_byte();

        let username = buffer_reader.read_string();
        let verification_key = buffer_reader.read_string();

        let magic_number = buffer_reader.read_byte();

        PlayerIdentification {
            sender,
            protocol_version,
            username,
            verification_key,
            magic_number,
        }
    }
}

impl NetworkPacket for PlayerIdentification {
    fn get_id(&self) -> u8 {
        Self::ID
    }
    fn get_size(&self) -> usize {
        Self::SIZE
    }

    fn get_sender_uid(&self) -> usize {
        self.sender
    }

    fn handle_receive(&self, core: &mut Core) {
        if let Some(mut player) = self.get_sender_mut(core) {
            player.set_name(&self.username);
            player.set_display_name(&self.username);

            player.set_uid(self.get_sender_uid());

            // TODO: Send CPE packets.

            let identify_packet = Box::new(ServerIdentification::new(
                0x07,
                String::from("Test"),
                String::from("A Rust server! +hax"),
                0x00,
            ));

            player.handle_packet(identify_packet);

            /*let map = Box::new(TestMap::new(Vec3D::new(32, 32, 32)));
            let world = World::new(map);*/

            let mut main_world = core.get_world_mut("main").unwrap(); // TODO: Give proper message (main does not exist.)

            core.send_map(player.value_mut(), main_world.value_mut());

            Core::static_log(&format!(
                "Player instantiated: {}",
                player.get_display_name()
            ));
        } // TODO: Handle case where player is not found or not instantiated.
    }
}


pub struct PlayerSetBlock {
    sender: usize,
    position: Vec3D,
    mode: u8,
    block: u8
}

impl PlayerSetBlock {
    pub const ID: u8 = 0x05;
    pub const SIZE: usize = 9;

    pub fn new(buffer_reader: &mut BufferReader, sender: usize) -> PlayerSetBlock {
        let x = buffer_reader.read_ushort();
        let y = buffer_reader.read_ushort();
        let z = buffer_reader.read_ushort();

        let mode = buffer_reader.read_byte();
        let block = buffer_reader.read_byte();

        PlayerSetBlock {
            sender,
            position: Vec3D::new(x, y, z),
            mode,
            block
        }
    }
}

impl NetworkPacket for PlayerSetBlock {
    fn get_id(&self) -> u8 {
        Self::ID
    }
    fn get_size(&self) -> usize {
        Self::SIZE
    }

    fn get_sender_uid(&self) -> usize {
        self.sender
    }

    fn handle_receive(&self, core: &mut Core) {
        if let Some(player) = self.get_sender(core) {
            // TODO: Make changes to actual map inside memory, to do so, we need to link player to map first.
            println!("Player's world: {}", player.get_world());
            if let Some(mut world) = core.get_world_mut(player.get_world()) {
                let mut destroy = false;

                if self.mode == 0x0 {
                    destroy = true;
                }

                world.set_block(&self.position, self.block, destroy);
            }

            Core::static_log(&format!(
                "Player sent a block change: \n(Vec3D): ({}, {}, {})",
                self.position.0, self.position.1, self.position.2
            ));
        } // TODO: Handle case where player is not found or not instantiated.
    }
}

pub struct PlayerPositionAndOrientation {
    sender: usize,
    player_id: u8,
    x: u16,
    y: u16,
    z: u16,
    pitch: u8,
    yaw: u8
}

impl PlayerPositionAndOrientation {
    pub const ID: u8 = 0x08;
    pub const SIZE: usize = 10;

    pub fn new(buffer_reader: &mut BufferReader, sender: usize) -> PlayerPositionAndOrientation {
        let player_id = buffer_reader.read_byte();

        let x = buffer_reader.read_ushort();
        let y = buffer_reader.read_ushort();
        let z = buffer_reader.read_ushort();

        let pitch = buffer_reader.read_byte();
        let yaw = buffer_reader.read_byte();

        

        PlayerPositionAndOrientation {
            sender,
            player_id,
            x, y, z,
            pitch,
            yaw,
        }
    }
}

impl NetworkPacket for PlayerPositionAndOrientation {
    fn get_id(&self) -> u8 {
        Self::ID
    }
    fn get_size(&self) -> usize {
        Self::SIZE
    }

    fn get_sender_uid(&self) -> usize {
        self.sender
    }

    fn handle_receive(&self, core: &mut Core) {
        if let Some(mut player) = self.get_sender_mut(core) {
            let transform = player.get_transform_mut();

            transform.set_pos(self.x, self.y, self.z);
            transform.set_pitch(self.pitch);
            transform.set_yaw(self.yaw);

            //println!("Player pos: {} {} {}", self.x, self.y, self.z);

            if let Some(mut world) = core.get_world_mut(player.get_world()) {
                // TODO: Update player to the others.
                for p in world.get_players() {
                    if *p != player.get_uid() {
                        if let Some(other) = core.get_player_by_uid(*p) {
                            // TODO: Send transform of other player.
                        }
                    }
                }
            }
        } // TODO: Handle case where player is not found or not instantiated.
    }
}

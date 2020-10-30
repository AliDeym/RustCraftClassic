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

use super::super::core::{events, BufferWriter, Core, Transform, Vec3D};
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

pub struct LevelFinalize {
    size: Vec3D,
}

impl LevelFinalize {
    pub const ID: u8 = 0x04;
    pub const SIZE: usize = 7;

    pub fn new(size: Vec3D) -> LevelFinalize {
        LevelFinalize { size }
    }
}

impl NetworkPacket for LevelFinalize {
    fn get_id(&self) -> u8 {
        Self::ID
    }
    fn get_size(&self) -> usize {
        Self::SIZE
    }

    fn handle_send(&self, buffer: &mut BufferWriter) {
        buffer.write_short(self.size.get_x());

        buffer.write_short(self.size.get_y());

        buffer.write_short(self.size.get_z());
    }
}

pub struct ServerSetBlock {
    position: Vec3D,
    block: u8,
}

impl ServerSetBlock {
    pub const ID: u8 = 0x06;
    pub const SIZE: usize = 8;

    pub fn new(position: Vec3D, block: u8) -> ServerSetBlock {
        ServerSetBlock { position, block }
    }
}

impl NetworkPacket for ServerSetBlock {
    fn get_id(&self) -> u8 {
        Self::ID
    }
    fn get_size(&self) -> usize {
        Self::SIZE
    }

    fn handle_send(&self, buffer: &mut BufferWriter) {
        buffer.write_short(self.position.get_x());

        buffer.write_short(self.position.get_y());

        buffer.write_short(self.position.get_z());

        buffer.write_byte(self.block);
    }
}

pub struct SpawnPlayer {
    player_id: i8,
    player_name: String,
    transform: Transform,
}

impl SpawnPlayer {
    pub const ID: u8 = 0x07;
    pub const SIZE: usize = 74;

    pub fn new(player_id: i8, player_name: String, transform: Transform) -> SpawnPlayer {
        SpawnPlayer {
            player_id,
            player_name,
            transform,
        }
    }
}

impl NetworkPacket for SpawnPlayer {
    fn get_id(&self) -> u8 {
        Self::ID
    }
    fn get_size(&self) -> usize {
        Self::SIZE
    }

    fn handle_send(&self, buffer: &mut BufferWriter) {
        buffer.write_sbyte(self.player_id);

        buffer.write_string(&self.player_name);

        buffer.write_transform(&self.transform);
    }
}

pub struct ServerPositionAndOrientation {
    player_id: i8,
    transform: Transform,
}

impl ServerPositionAndOrientation {
    pub const ID: u8 = 0x08;
    pub const SIZE: usize = 10;

    pub fn new(player_id: i8, transform: Transform) -> ServerPositionAndOrientation {
        ServerPositionAndOrientation {
            player_id,
            transform,
        }
    }
}

impl NetworkPacket for ServerPositionAndOrientation {
    fn get_id(&self) -> u8 {
        Self::ID
    }
    fn get_size(&self) -> usize {
        Self::SIZE
    }

    fn handle_send(&self, buffer: &mut BufferWriter) {
        buffer.write_sbyte(self.player_id);

        buffer.write_transform(&self.transform);
    }
}

pub struct DespawnPlayer {
    player_id: i8,
}

impl DespawnPlayer {
    pub const ID: u8 = 0x0c;
    pub const SIZE: usize = 2;

    pub fn new(player_id: i8) -> DespawnPlayer {
        DespawnPlayer { player_id }
    }
}

impl NetworkPacket for DespawnPlayer {
    fn get_id(&self) -> u8 {
        Self::ID
    }
    fn get_size(&self) -> usize {
        Self::SIZE
    }

    fn handle_send(&self, buffer: &mut BufferWriter) {
        buffer.write_sbyte(self.player_id);
    }
}

pub struct Message {
    player_id: i8,
    message: String,
}

impl Message {
    pub const ID: u8 = 0x0d;
    pub const SIZE: usize = 66;

    pub fn new(player_id: i8, message: String) -> Message {
        Message { player_id, message }
    }
}

impl NetworkPacket for Message {
    fn get_id(&self) -> u8 {
        Self::ID
    }
    fn get_size(&self) -> usize {
        Self::SIZE
    }

    fn handle_send(&self, buffer: &mut BufferWriter) {
        buffer.write_sbyte(self.player_id);

        buffer.write_string(&self.message);
    }
}

pub struct DisconnectPlayer {
    sender: usize,
    reason: String,
}

impl DisconnectPlayer {
    pub const ID: u8 = 0x0e;
    pub const SIZE: usize = 65;

    pub fn new(sender: usize, reason: String) -> DisconnectPlayer {
        DisconnectPlayer { sender, reason }
    }
}

impl NetworkPacket for DisconnectPlayer {
    fn get_id(&self) -> u8 {
        Self::ID
    }
    fn get_size(&self) -> usize {
        Self::SIZE
    }
    fn get_sender_uid(&self) -> usize {
        self.sender
    }

    fn handle_send(&self, buffer: &mut BufferWriter) {
        buffer.write_string(&self.reason);
    }

    // Received from Network Thread.
    fn handle_receive(&self, core: &mut Core) {
        let mut player_id = 0;
        let mut found = false;

        if let Some(mut ply) = self.get_sender_mut(core) {
            // Player is found.
            player_id = ply.get_uid();
            found = true;

            if let Some(mut world) = core.get_world_mut(ply.get_world()) {
                // Despawn player from others.
                for ply_id in world.get_players() {
                    if *ply_id != ply.get_uid() {
                        if let Some(mut other) = core.get_player_by_uid_mut(*ply_id) {
                            let packet = Box::new(DespawnPlayer::new(ply.get_uid() as i8));

                            other.handle_packet(packet);
                        }
                    }
                }

                // Remove from current world.
                world.remove_player(ply.get_uid());
            }

            // Send message to everyone else. At this point, player entity is still valid. (Network may not be available though).
            events::player::on_left(core, ply.as_mut());
        }

        // Finally, remove from core (If player existed):
        core.remove_player_by_uid(player_id);
    }
}

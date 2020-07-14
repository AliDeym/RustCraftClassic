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

use super::super::core::{BufferReader, Core};
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

    pub fn new(
        sender: usize,
        protocol_version: u8,
        username: String,
        verification_key: String,
        magic_number: u8,
    ) -> PlayerIdentification {
        PlayerIdentification {
            sender,
            protocol_version,
            username,
            verification_key,
            magic_number,
        }
    }

    pub fn from(buffer_reader: &mut BufferReader, sender: usize) -> PlayerIdentification {
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
                0x07, String::from("Test"), String::from("A Rust server!"), 0x00
            ));

            player.handle_packet(identify_packet);

            core.send_map(player.value_mut(), "any.dat");

            Core::static_log(&format!("Player instantiated: {}", player.get_display_name()));
        } // TODO: Handle case where player is not found or not instantiated.
    }
}

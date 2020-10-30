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

use std::io::Write;
use std::net::TcpStream;

use super::super::network::{Message, NetworkPacket, ServerPositionAndOrientation};
use super::events;
use super::{BufferWriter, Core, Transform};

pub trait Player {
    fn set_uid(&mut self, id: usize);
    fn get_uid(&self) -> usize;

    fn set_name(&mut self, _name: &str) {}
    fn get_name(&self) -> &str;

    fn set_display_name(&mut self, _name: &str) {}
    fn get_display_name(&self) -> &str;

    fn set_world(&mut self, _map: &str) {}
    fn get_world(&self) -> &str {
        "main"
    }

    fn get_transform(&self) -> &Transform;
    fn get_transform_mut(&mut self) -> &mut Transform;

    fn update_transform(&mut self, transform: Transform) {}

    fn is_console(&self) -> bool {
        false
    }

    fn handle_packet(&mut self, packet: Box<dyn NetworkPacket>);

    fn kill(&mut self) {}
    fn send_message(&mut self, message: &str) {
        // Message sent to console.
        Core::static_log(message);
    }

    fn try_join_world(&mut self, core: &Core, map: &str) {}
}

pub struct NetworkPlayer {
    uid: usize,
    stream: TcpStream,

    username: String,
    nickname: String,

    world: String,

    transform: Transform,
}

impl NetworkPlayer {
    /// Creates a network based player.
    pub fn new(uid: usize, stream: TcpStream) -> NetworkPlayer {
        let default_name = String::from("Uninitialized Player");

        NetworkPlayer {
            uid,
            stream,
            nickname: default_name.clone(),
            username: default_name,
            world: String::from(""),

            transform: Transform::default(),
        }
    }

    /*pub fn listen_network(&self) {
        if let Ok(pool) = self.threadpool.lock() {
            pool.execute(move || {
                let mut buffer = vec![];

                loop {
                    match self.receiving_stream.read_to_end(&mut buffer) {
                        Ok(_) => {
                            buffer.clear(); // Clean up the buffer after receiving a packet.
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
                        Err(e) => {
                            Core::static_log(&format!("IO error on player received: {}", e));
                        }
                    }
                }
            });
        }
    }*/
}

impl Player for NetworkPlayer {
    fn set_uid(&mut self, id: usize) {
        self.uid = id;
    }
    fn get_uid(&self) -> usize {
        self.uid
    }

    fn get_transform(&self) -> &Transform {
        &self.transform
    }
    fn get_transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    fn update_transform(&mut self, transform: Transform) {
        // TODO: Remove double cloning.
        self.transform = transform;

        let packet = Box::new(ServerPositionAndOrientation::new(
            -1,
            self.transform.clone(),
        ));

        self.handle_packet(packet);
    }

    fn set_name(&mut self, name: &str) {
        self.username = String::from(name);
    }
    fn get_name(&self) -> &str {
        &self.username
    }

    fn set_display_name(&mut self, name: &str) {
        self.nickname = String::from(name);
    }
    fn get_display_name(&self) -> &str {
        &self.nickname
    }

    fn get_world(&self) -> &str {
        &self.world
    }

    fn set_world(&mut self, map: &str) {
        self.world = String::from(map);
    }

    fn kill(&mut self) {
        println!("Player died.")
    }

    fn send_message(&mut self, message: &str) {
        let packet = Box::new(Message::new(0, String::from(message)));

        self.handle_packet(packet);
    }

    fn try_join_world(&mut self, core: &Core, map: &str) {
        // Map found:
        if let Some(mut map) = core.get_world_mut(map) {
            // Check events, false means event was not surpressed.
            if !events::world::on_join(&core, self, &mut map) {
                core.send_map(self, &mut map);

                events::world::on_joined(&core, self, map);
            }
        } else {
            // See if map exists in map folder, then try to load it.
            if core.try_load_map(self, map) {
                if let Some(mut map) = core.get_world_mut(map) {
                    // Check events, false means event was not surpressed.
                    if !events::world::on_join(&core, self, &mut map) {
                        core.send_map(self, &mut map);

                        events::world::on_joined(&core, self, map);
                    }
                }
            } else {
                // Map could not be loaded.
                events::world::on_notfound(core, self, map);
            }
        }
    }

    fn handle_packet(&mut self, packet: Box<dyn NetworkPacket>) {
        let buffer_size = packet.get_size();
        let packet_id = packet.get_id();

        let mut buffer = BufferWriter::new(buffer_size);

        buffer.write_byte(packet_id);

        packet.handle_send(&mut buffer);

        if let Ok(sent_bytes) = self.stream.write(buffer.get_data()) {
            // TODO: Check writing and error of packet sending.
            /*Core::static_log(&format!(
                "Sent Packet ID \"{}\", \"{}\" bytes.",
                packet.get_id(),
                sent_bytes
            ));*/
        }
    }
}

pub struct Console {
    // A position is needed for transform method, which does nothing actually though.
    transform: Transform,
}

impl Console {
    pub fn new() -> Console {
        Console {
            transform: Transform::default(),
        }
    }
}

impl Player for Console {
    fn get_transform(&self) -> &Transform {
        &self.transform
    }
    fn get_transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    fn set_uid(&mut self, _id: usize) {}
    fn get_uid(&self) -> usize {
        0
    }

    fn get_name(&self) -> &str {
        "Console"
    }

    fn get_display_name(&self) -> &str {
        "&0Console"
    }

    fn handle_packet(&mut self, _packet: Box<dyn NetworkPacket>) {}
}

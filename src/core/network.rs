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

use std::io::{self, Read};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::mpsc::Sender;
use std::time::Duration;

use threadpool::ThreadPool;

use super::super::network::*;
use super::{BufferReader, Core, NetworkPlayer, PlayerList};

const HOSTNAME: &str = "0.0.0.0";
const TIMEOUT_TIME: u64 = 30; // in Seconds.

pub struct Network {
    listener: TcpListener,
    net_workers: ThreadPool,
}

impl Network {
    /// Instantiates a Network Instance on specific port.
    pub fn new(port: usize, threadsize: usize) -> Network {
        let listener = TcpListener::bind(format!("{}:{}", HOSTNAME, port)).unwrap();

        Network {
            listener,
            net_workers: ThreadPool::new(threadsize),
        }
    }

    /// Locks the current thread, waiting to receive connections.
    pub fn listen(&self, players_arc: PlayerList, core_tx: Sender<Box<dyn NetworkPacket + Send>>) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    let try_clone_stream = || -> Result<TcpStream, io::Error> {
                        let timeout_duration = Some(Duration::from_secs(TIMEOUT_TIME));

                        stream.set_write_timeout(timeout_duration)?;
                        stream.set_read_timeout(timeout_duration)?;

                        // This program needs sockets to be non-blocking in order for ThreadPool to work properly.
                        stream.set_nonblocking(true)?; // Panic in case we cannot set our stream into non-blocking mode.

                        let cloned = stream.try_clone()?; // Clone the stream to simultaneously send to and receive from players.

                        Ok(cloned)
                    };

                    match try_clone_stream() {
                        Ok(mut receiver) => {
                            let players = players_arc.clone();

                            let mut found_id = 0;
                            'search: for _ in 1..(std::i8::MAX) as usize - 1 {
                                found_id += 1;

                                if players.contains_key(&(found_id as usize)) {
                                    continue;
                                }

                                break 'search;
                            }

                            let player_uid = found_id;
                            let uid_copy = player_uid.clone();

                            let tx = core_tx.clone();

                            self.net_workers.execute(move || {
                                let mut buffer = vec![0xff; 256];

                                loop {
                                    match receiver.read(&mut buffer) {
                                        Ok(size) => {
                                            if size <= 0 {
                                                // Connection has been closed.
                                                // Inform the core of player's disconnecction so the proper action can be taken.
                                                // TODO: Send a disconnect packet to core, with sender ID 0 (console). (Dispose NEEDED!)
                                                Core::static_log(&format!(
                                                    "Player with uid \"{}\" disconnected.",
                                                    uid_copy
                                                ));

                                                receiver.shutdown(Shutdown::Both).ok();

                                                break;
                                            }

                                            let mut buffer_reader = BufferReader::new(&buffer);

                                            while buffer_reader.get_index() < size {
                                                // Read the packet's op_code.
                                                // TODO: Check the buffer for corrupt data.
                                                let op_code = buffer_reader.read_byte();

                                                // Handle receiving packets here, and send objects to core.
                                                // TODO: match should return a packet, send must be done outside the match.
                                                match op_code {
                                                    PlayerIdentification::ID => {
                                                        println!("SIZE: {}, BUFFER: {}", size, buffer_reader.get_index());
                                                        let identify_packet =
                                                            PlayerIdentification::new(
                                                                &mut buffer_reader,
                                                                player_uid,
                                                            );

                                                        tx.send(Box::new(identify_packet)).unwrap();
                                                    }
                                                    PlayerSetBlock::ID => {
                                                        println!("ID {} sent a setblock with {} bytes data", player_uid, size);
                                                        let setblock_packet = PlayerSetBlock::new(
                                                            &mut buffer_reader,
                                                            player_uid,
                                                        );

                                                        tx.send(Box::new(setblock_packet)).unwrap();
                                                    }
                                                    PlayerPositionAndOrientation::ID => {
                                                        let packet =
                                                            PlayerPositionAndOrientation::new(
                                                                &mut buffer_reader,
                                                                player_uid,
                                                            );
                                                        tx.send(Box::new(packet)).unwrap();
                                                    }
                                                    _ => {}
                                                }
                                                //buffer.clear(); // Clean up the buffer after receiving a packet.
                                            }
                                        }
                                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
                                        Err(e) => {
                                            Core::static_log(&format!(
                                                "IO error on player received: {}",
                                                e
                                            ));
                                        }
                                    }
                                }
                            });
                            // TODO: Let the core edit players. Insertion should be move into core, not network.
                            let spawned_player = NetworkPlayer::new(uid_copy, stream);
                            players.insert(uid_copy, Box::new(spawned_player));
                        }
                        Err(e) => {
                            Core::static_log(&format!(
                                "Error parameterizing network stream: {}",
                                e
                            ));
                        }
                    }
                }
                Err(err) => {
                    Core::static_log(&format!("Error receiving network stream: {}", err));
                }
            }
        }
    }
}

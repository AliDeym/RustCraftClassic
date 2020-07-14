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

use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::Sender;
use std::time::Duration;


use threadpool::ThreadPool;


use super::super::network::*;
use super::{Core, NetworkPlayer, PlayerList};

const HOSTNAME: &str = "0.0.0.0";
const TIMEOUT_TIME: u64 = 30; // in Seconds.


pub struct BufferWriter {
    buffer: Vec<u8>
}

impl BufferWriter {
    pub fn new(size: usize) -> BufferWriter {
        BufferWriter {
            buffer: Vec::<u8>::with_capacity(size)
        }
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.buffer
    }

    pub fn write_byte(&mut self, data: u8) {
        self.buffer.push(data);
    }

    pub fn write_sbyte(&mut self, data: i8) {
        self.buffer.push(data as u8);
    }

    pub fn write_short(&mut self, data: u16) {
        let bytes = data.to_be_bytes();

        self.buffer.extend(&bytes[..]);
    }

    pub fn write_string(&mut self, data: &str) {
        let mut char_iter = data.as_bytes().iter();

        for _ in 0..64 {
            self.buffer.push(*char_iter.next().unwrap_or(&b' ') as u8);
        }
    }

    pub fn write_array(&mut self, data: &Vec<u8>) {
        if data.len() > 1024 {
            self.buffer.extend(data.split_at(1024).0);

            return;
        }

        self.buffer.extend(data);
    }
}


pub struct BufferReader<'a> {
    index: usize,
    buffer: &'a Vec<u8>,
}


impl<'a> BufferReader<'a> {
    pub fn new(buffer: &'a Vec<u8>) -> BufferReader {
        BufferReader { index: 0, buffer }
    }

    pub fn read_byte(&mut self) -> u8 {
        self.index += 1;

        *self.buffer.get(self.index - 1).unwrap_or(&0)
    }

    pub fn read_sbyte(&mut self) -> i8 {
        self.index += 1;

        *self.buffer.get(self.index - 1).unwrap_or(&0) as i8
    }

    pub fn read_ushort(&mut self) -> u16 {
        self.index += 2;

        let b1 = self.read_byte();
        let b2 = self.read_byte();
        
        (b1 as u16) << 8 | b2 as u16
    }

    pub fn read_string(&mut self) -> String {
        let grabbed = String::from_utf8(self.buffer[self.index..self.index + 64].to_vec())
            .unwrap_or(String::from(""));

        self.index += 64;

        String::from(grabbed.trim())
    }
}

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

                            // POSSIBLE BUG: UID exceeds usize limit after a certain amount of time.
                            let player_uid = (*players).iter().last().map_or(1, |x| {
                                if *x.key() >= std::usize::MAX - 1 {
                                    1
                                } else {
                                    (*x.key()) + 1
                                }
                            });
                            let uid_copy = player_uid.clone();

                            let tx = core_tx.clone();

                            self.net_workers.execute(move || {
                                let mut buffer = vec![0; 256];

                                loop {
                                    match receiver.read(&mut buffer) {
                                        Ok(size) => {
                                            if size <= 0 {
                                                // Connection has been closed.
                                                // Inform the core of player's disconnecction so the proper action can be taken.
                                                

                                                break;
                                            }

                                            let result_opcode = buffer.get(0);

                                            if let None = result_opcode {
                                                //tx.send(Box::new())
                                                // TODO: Send a disconnect packet to server, so it knows player has disconnected.
                                                // which will drop the player.
                                                // POTENTIAL RISK: Another player sending disconenct to the server
                                                // causing other players to get kicked.
                                                break;
                                            }

                                            let op_code = result_opcode.unwrap();
                                            let mut buffer_reader = BufferReader::new(&buffer);
                                            // Ignore the first byte, as we already handle it with op_code variable.
                                            buffer_reader.read_byte();

                                            // Handle receiving packets here, and send objects to core.
                                            match *op_code {
                                                PlayerIdentification::ID => {

                                                    let identify_packet =
                                                        PlayerIdentification::from(
                                                            &mut buffer_reader,
                                                            player_uid,
                                                        );

                                                    

                                                    tx.send(Box::new(identify_packet)).unwrap();
                                                }
                                                _ => {}
                                            }
                                            //buffer.clear(); // Clean up the buffer after receiving a packet.
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

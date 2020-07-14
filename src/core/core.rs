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

use std::sync::{Arc, mpsc::{channel, Receiver, Sender}};
use std::thread;

use dashmap::mapref::one::{Ref, RefMut};
use dashmap::DashMap;
use num_cpus;

use super::super::network::{NetworkPacket, LevelInitialize, LevelDataChunk};
use super::{Console, Network, Player};

pub type PlayerList = Arc<DashMap<usize, Box<dyn Player + Send + Sync>>>;

pub struct Core {
    pub threadsize: usize,

    players: PlayerList,

    tx: Option<Sender<Box<dyn NetworkPacket + Send>>>,
    rx: Option<Receiver<Box<dyn NetworkPacket + Send>>>,
}

impl Core {
    /// Creates a new rcclassic Core with number of threads to handle player connections.
    pub fn new(mut threadsize: usize) -> Core {
        if threadsize <= 0 {
            threadsize = num_cpus::get_physical();

            Core::static_log("Thread size cannot be 0 or less. Using default (# CPU Cores).");
        }

        let players: PlayerList = Arc::new(DashMap::new());

        (*players).insert(0, Box::new(Console::new()));

        Core {
            threadsize,

            players,

            tx: None,
            rx: None,
        }
    }

    /// Logs into the standard output as well as log file, without a core instance.
    pub fn static_log(message: &str) -> String {
        let log = format!("[{}] {}", "TIME-HERE", message);

        println!("{}", &log);

        log
    }

    /// Logs messages into standard output and log file.
    pub fn log(&self, message: &str) -> String {
        Core::static_log(&message)
    }

    /// Generates the required memory channels for core.
    /// Basically, instantiates Receiving and Sending channel of the Core.
    pub fn generate_mem_chans(&mut self) {
        let (tx, rx) = channel::<Box<dyn NetworkPacket + Send>>();

        self.tx = Some(tx);
        self.rx = Some(rx);
    }

    /// Takes the receiving end from core, leaving it without any receiver. (Moves it)
    ///
    /// # Panics
    ///
    /// Panics if receiver is not instantiated.
    pub fn receiver_take(&mut self) -> Receiver<Box<dyn NetworkPacket + Send>> {
        if let None = self.rx {
            panic!(Core::static_log(
                "Cannot take Receiver from core; Receiver not present in core's memory channel."
            ));
        }

        self.rx.take().unwrap()
    }

    /// Moves the sending end of the core out of it.
    ///
    /// # Panics
    ///
    /// Panics if sender is already taken or not instantiated.
    pub fn sender_take(&mut self) -> Sender<Box<dyn NetworkPacket + Send>> {
        if let None = self.tx {
            panic!(Core::static_log(
                "Cannot take Sender from core; Sender not present in core's memory channel."
            ));
        }

        self.tx.take().unwrap()
    }

    /// Creates a clone from the sender.
    ///
    /// # Panics
    ///
    /// Panics if sender is not instantiated or is taken (is not present in class).
    pub fn sender_clone(&self) -> Sender<Box<dyn NetworkPacket + Send>> {
        if let None = self.tx {
            panic!(Core::static_log(
                "Cannot clone a Sender from core; Sender not present in core's memory channel."
            ));
        }

        self.tx.clone().unwrap()
    }

    pub fn get_player_by_uid<'core>(
        &'core self,
        uid: usize,
    ) -> Option<Ref<'_, usize, Box<dyn Player + Send + Sync>>> {
        if self.players.contains_key(&uid) {
            self.players.get(&uid)
        } else {
            None
        }
    }

    pub fn get_player_by_uid_mut<'core>(
        &'core self,
        uid: usize,
    ) -> Option<RefMut<'_, usize, Box<dyn Player + Send + Sync>>> {
        if self.players.contains_key(&uid) {
            self.players.get_mut(&uid)
        } else {
            None
        }
    }

    pub fn send_map(&self, player: &mut Box<dyn Player + Send + Sync>, map: &str) {
        // TODO: Send actual map.
        let mut data1 = Vec::<u8>::new();

        for x in 0..2048 {
            data1.push(0x00);
        }
        // Test packet.

        let data2 = data1.split_off(1024);
        
        player.handle_packet(Box::new(LevelInitialize::new()));
        player.handle_packet(Box::new(LevelDataChunk::new(1024, data1, 20)));
        player.handle_packet(Box::new(LevelDataChunk::new(1024, data2, 20)));
    }

    pub fn network_listen(&mut self) {
        let core_tx = self.sender_take();
        let players_arc = self.players.clone();

        let thread_size = self.threadsize;

        thread::spawn(move || {
            let network = Network::new(27015, thread_size);

            network.listen(players_arc, core_tx);
        });
    }

    /// Listens for network connection.
    pub fn handle_received_packets(&mut self) {
        let receiver = self.receiver_take();

        for message in receiver {
            self.log(&format!("Received a packet with id: {}", message.get_id()));
            
            message.handle_receive(self);
        }

        panic!(self.log("Receiving memory has stopped unexpectedly."));
    }
}

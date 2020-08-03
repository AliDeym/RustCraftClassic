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

use std::io::prelude::*;

use std::sync::{
    mpsc::{channel, Receiver, Sender},
    Arc,
};
use std::thread;

use chashmap::{CHashMap, WriteGuard, ReadGuard};
use flate2::write::GzEncoder;
use flate2::Compression;
use num_cpus;
use chrono::prelude::*;
use chrono::Local;

use super::super::network::*;
use super::{Console, Network, Player, MemoryMap, Vec3D, World, Transform};

pub type PlayerList = Arc<CHashMap<usize, Box<dyn Player + Send + Sync>>>;
pub type WorldList = Arc<CHashMap<String, World>>;

pub struct Core {
    pub threadsize: usize,

    players: PlayerList,
    worlds: WorldList,

    tx: Option<Sender<Box<dyn NetworkPacket + Send>>>,
    rx: Option<Receiver<Box<dyn NetworkPacket + Send>>>,
}

impl Core {
    /// Creates a new rcclassic Core with number of threads to handle player connections.
    /// 'threadsize' can be left 0 to use the the physical core count.
    pub fn new(mut threadsize: usize) -> Core {
        if threadsize <= 0 {
            threadsize = num_cpus::get_physical();

            Core::static_log("Thread size cannot be 0 or less. Using default (# CPU Cores).");
        }

        Core::static_log(&format!("Initializing the core with '{}' threads...", threadsize));

        let players: PlayerList = Arc::new(CHashMap::new());

        (*players).insert(0, Box::new(Console::new()));

        let worlds: WorldList = Arc::new(CHashMap::new());

        (*worlds).insert(
            String::from("main"),
            World::new(
                String::from("main"),
                Box::new(MemoryMap::new(Vec3D::new(64, 16, 64))),
            ),
        );

        Core::static_log("Core has ben set up, waiting for network.");

        Core {
            threadsize,

            players,
            worlds,

            tx: None,
            rx: None,
        }
    }

    /// Logs into the standard output as well as log file, without a core instance.
    pub fn static_log(message: &str) -> String {
        let time_now = Local::now();
        let log = format!("[{}] {}", time_now.format("%H:%M:%S"), message);

        println!("{}", &log);

        // TODO: Log into a log file.
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

    // TODO: Change player by_uid to get_player
    /// Returns a player reference by uid. UID 0 can be used to get 'Console'.
    pub fn get_player_by_uid<'core>(
        &'core self,
        uid: usize,
    ) -> Option<ReadGuard<'_, usize, Box<dyn Player + Send + Sync>>> {
        if self.players.contains_key(&uid) {
            self.players.get(&uid)
        } else {
            None
        }
    }

    // TODO: Change player by_uid to get_player
    /// Returns a mutable player reference by uid. UID 0 can be used to get 'Console'.
    pub fn get_player_by_uid_mut<'core>(
        &'core self,
        uid: usize,
    ) -> Option<WriteGuard<usize, Box<dyn Player + Send + Sync>>> {
        if self.players.contains_key(&uid) {
            self.players.get_mut(&uid)
        } else {
            None
        }
    }

    pub fn get_world<'core>(&'core self, name: &str) -> Option<ReadGuard<String, World>> {
        if self.worlds.contains_key(name) {
            self.worlds.get(name)
        } else {
            None
        }
    }

    pub fn get_world_mut<'core>(&'core self, name: &str) -> Option<WriteGuard<String, World>> {
        if self.worlds.contains_key(name) {
            self.worlds.get_mut(name)
        } else {
            None
        }
    }

    pub fn send_map(&self, mut player: WriteGuard<usize, Box<dyn Player + Send + Sync>>, map: &mut World) {
        if let Some(mut current_world) = self.get_world_mut(player.get_world()) {
            current_world.remove_player(player.get_uid());

            for players in current_world.get_players() {
                // TODO: Send entity remove packet to existing players.
            }
        }

        // TODO: Send actual map.
        map.add_player(player.get_uid());
        player.set_world(map.get_name());

        let mut gz = GzEncoder::new(Vec::new(), Compression::default());

        // TODO: Handle both write_all and finish results efficiently.
        let size = &(map.get_chunks().len() as u32).to_be_bytes();
        gz.write_all(size).unwrap();
        gz.write_all(map.get_chunks()).unwrap();
        let gz_data = gz.finish().unwrap();

        player.handle_packet(Box::new(LevelInitialize::new()));

        let chunks = gz_data.chunks(1024);
        let total_chunks = chunks.len();

        for (i, chunk) in chunks.enumerate() {
            let mut chunk_data = chunk.to_vec();

            if chunk.len() < 1024 {
                for _ in chunk.len()..1024 {
                    chunk_data.push(0x0);
                }
            }

            player.handle_packet(Box::new(LevelDataChunk::new(
                chunk.len() as u16,
                chunk_data,
                (i / total_chunks * 100) as u8,
            )));
        }

        player.handle_packet(Box::new(LevelFinalize::new(
            *map.get_size()
        )));

        // TODO: Set spawn points for player, as well as send entity creation to players in world.
        let transform = Transform::new(map.get_spawnarea(), 255, 0);
        let name = String::from(player.get_display_name());
        let uid = player.get_uid();

        // Sending spawn area.
        player.handle_packet(Box::new(SpawnPlayer::new(-1, name.clone(), transform.clone())));
        //drop(player); // Dropping to stop deadlocks because we want to iterate through the list.
        for pid in map.get_players() {
            if *pid != uid {
                if let Some(mut other) = self.get_player_by_uid_mut(*pid) {
                    // Let other players know about the joining player.
                    other.handle_packet(Box::new(SpawnPlayer::new(uid as i8, name.clone(), transform.clone())));

                    // Let the joining player now about the other(s).
                    player.handle_packet(Box::new(SpawnPlayer::new(other.get_uid() as i8, String::from(other.get_display_name()), transform.clone())));
                }
            }
        }
    }

    /// Starts a thread which listens for incoming connections.
    pub fn network_listen(&mut self) {
        let core_tx = self.sender_take();
        let players_arc = self.players.clone();

        let thread_size = self.threadsize;

        thread::spawn(move || {
            let network = Network::new(27015, thread_size);

            network.listen(players_arc, core_tx);
        });
    }

    /// Listens for network packets over the memory channel.
    pub fn handle_received_packets(&mut self) {
        let receiver = self.receiver_take();

        for message in receiver {
            //self.log(&format!("Received a packet with id: {}", message.get_id()));

            message.handle_receive(self);
        }

        panic!(self.log("FATAL ERROR: Receiving memory has stopped unexpectedly."));
    }
}

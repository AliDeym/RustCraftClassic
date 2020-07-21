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

mod core;
mod map;
mod network;
mod player;
mod util;
mod world;

pub use self::core::*;
pub use self::map::*;
pub use self::network::*;
pub use self::player::*;
pub use self::util::*;
pub use self::world::*;

#[cfg(test)]
mod test_core {
    use super::super::network::*;
    use super::*;

    use std::thread;

    #[test]
    /// Tests whether or not core overrides threadsize in case it is zero.
    pub fn create_default_core() {
        Core::new(0);
    }

    #[test]
    /// Tests core's memory channels in both receiving and sending ends.
    pub fn mem_test() {
        let mut core = Core::new(4);

        core.generate_mem_chans();

        let sender = core.sender_take();
        let receiver = core.receiver_take();

        /*let sending_object = Box::new(PlayerIdentification {});

        thread::spawn(move || {
            sender.send(sending_object).unwrap();
        });

        for packet in receiver.iter().take(1) {
            let simulation_packet = PlayerIdentification {};
            let boxed_packet: Box<dyn NetworkPacket> = Box::new(simulation_packet);

            assert!(*packet == *boxed_packet);
        }*/
    }
}

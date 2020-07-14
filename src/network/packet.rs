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

use std::cmp::PartialEq;

use dashmap::mapref::one::{Ref, RefMut};

use super::super::core::{BufferWriter, Core, Player};

pub trait NetworkPacket {
    fn get_id(&self) -> u8;
    fn get_size(&self) -> usize;
    
    fn get_sender_uid(&self) -> usize { 0 }

    fn get_sender<'a>(&self, core: &'a Core) -> Option<Ref<'a, usize, Box<dyn Player + Send + Sync>>> {
        let uid = self.get_sender_uid();

        core.get_player_by_uid(uid)
    }

    fn get_sender_mut<'a>(&self, core: &'a Core) -> Option<RefMut<'a, usize, Box<dyn Player + Send + Sync>>> {
        let uid = self.get_sender_uid();

        core.get_player_by_uid_mut(uid)
    }

    fn handle_receive(&self, core: &mut Core) {}

    fn handle_send(&self, buffer: &mut BufferWriter) {}

}
/*
impl PartialEq<dyn NetworkPacket> for dyn NetworkPacket {
    fn eq(&self, other: &dyn NetworkPacket) -> bool {
        self.get_id() == other.get_id()
    }
}*/
// TODO: Implement for dyn NetworkPacket + Send
// TODO: Implement all possible checks.
impl PartialEq<dyn NetworkPacket + 'static> for dyn NetworkPacket + Send {
    fn eq(&self, other: &dyn NetworkPacket) -> bool {
        self.get_id() == other.get_id()
    }
}

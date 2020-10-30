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
use super::super::super::network::*;
use super::super::{Core, Player};
use chashmap::WriteGuard;

fn join_welcome(core: &Core, player: &mut (dyn Player), surpress: &mut bool) {
    let nick = String::from(player.get_display_name());

    core.broadcast_message(player, &format!("{} &6has joined the server!", nick));
}

fn leave_goodbye(core: &Core, player: &mut (dyn Player), surpress: &mut bool) {
    let nick = String::from(player.get_display_name());

    core.broadcast_message(player, &format!("{} &6has left the server.", nick));
}

// Called after player joined the server.
pub fn on_joined(core: &Core, player: &mut (dyn Player)) -> bool {
    let mut surpress = false;

    join_welcome(core, player, &mut surpress);

    surpress
}

// Called after player left the server.
// Player is valid, but network stream is not.
pub fn on_left(core: &Core, player: &mut (dyn Player)) -> bool {
    let mut surpress = false;

    leave_goodbye(core, player, &mut surpress);

    surpress
}

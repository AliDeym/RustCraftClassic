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
use super::super::{Core, Player, Vec3D, World};
use chashmap::WriteGuard;

// TODO: Add world load event.
// TODO: Add world unload event.

fn notify_join_world(
    core: &Core,
    player: &mut (dyn Player),
    world: &mut World,
    surpress: &mut bool,
) {
    // Event already handled.
    if *surpress {
        return;
    }

    let nick_name = String::from(player.get_display_name());

    core.broadcast_message(
        player,
        &format!("{} &8joined the world \"{}\".", nick_name, world.get_name()),
    );
}

fn notify_not_found(core: &Core, player: &mut (dyn Player), world_name: &str, surpress: &mut bool) {
    // Event already handled.
    if *surpress {
        return;
    }

    player.send_message(&format!("&8The world \"{}\" does not exist.", world_name));
}

fn readonly_build(
    core: &Core,
    player: &mut (dyn Player),
    world: &mut World,
    position: Vec3D,
    block: u8,
    destroy: bool,
    surpress: &mut bool,
) {
    // Event already handled.
    if *surpress {
        return;
    }

    player.send_message("&8Server is on read-only state.");
    *surpress = true;
}

/// Called when user tries to set block. surpress to prevent saving on underlying struct.
pub fn on_setblock(
    core: &Core,
    player: &mut (dyn Player),
    world: &mut World,
    position: Vec3D,
    block: u8,
    destroy: bool,
) -> bool {
    let mut surpress = false;

    readonly_build(core, player, world, position, block, destroy, &mut surpress);

    surpress
}

/// Called when user tries to join a world that does not exist.
pub fn on_notfound(core: &Core, player: &mut (dyn Player), world: &str) {
    let mut surpress = false;

    notify_not_found(core, player, world, &mut surpress);
}

/// Called before player joins a world. Surpressing the event here prevents joining.
pub fn on_join(
    core: &Core,
    mut player: &mut (dyn Player),
    world: &mut WriteGuard<String, World>,
) -> bool {
    let mut surpress = false;

    // TODO: More methods to prevent/check joining.

    surpress
}

/// Called after player has joined a world.
pub fn on_joined(
    core: &Core,
    player: &mut (dyn Player),
    mut world: WriteGuard<String, World>,
) -> bool {
    let mut surpress = false;

    notify_join_world(core, player, &mut world, &mut surpress);

    surpress
}

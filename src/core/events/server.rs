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
use super::super::{Core, Player, SyncPlayer};
use chashmap::WriteGuard;

fn help_command(core: &Core, player: &mut (dyn Player), message: &str, surpress: &mut bool) {
    // Event already handled.
    if *surpress {
        return;
    }

    if message.eq_ignore_ascii_case("/help") {
        player.send_message("&cMC Classic Written in Rust by Ali Deym.");
        player.send_message("&7/main - Go to main.");
        player.send_message("&7/join {map} - Joins or loads the specified map. (/j)");
        player.send_message("&7/tp {player} - Go to {player} if any.");
        player.send_message("&7/players - List of online players.");

        *surpress = true;
    }
}

fn main_command(core: &Core, player: &mut (dyn Player), message: &str, surpress: &mut bool) {
    // Event already handled.
    if *surpress {
        return;
    }

    if message.eq_ignore_ascii_case("/main") {
        player.try_join_world(core, "main");

        *surpress = true;
    }
}

fn join_command(core: &Core, player: &mut (dyn Player), message: &str, surpress: &mut bool) {
    // Event already handled.
    if *surpress {
        return;
    }

    if message.to_lowercase().starts_with("/j") || message.to_lowercase().starts_with("/join") {
        let message_lowercase = message.to_lowercase();
        let world_name = message_lowercase.split_ascii_whitespace().last();

        if world_name.is_none() {
            help_command(core, player, "/help", surpress);

            return;
        }

        player.try_join_world(core, world_name.unwrap());

        *surpress = true;
    }
}

fn tp_command(core: &Core, player: &mut (dyn Player), message: &str, surpress: &mut bool) {
    // Event already handled.
    if *surpress {
        return;
    }

    if message.to_lowercase().starts_with("/tp") || message.to_lowercase().starts_with("/teleport")
    {
        *surpress = true;

        let message_lowercase = message.to_lowercase();
        let query: String = message_lowercase.split_ascii_whitespace().skip(1).collect();

        let mut found_position = None;
        let mut other_world = String::from(player.get_world());
        // TODO: Find a better way to iterate.
        for i in 0..128 {
            if i != player.get_uid() {
                if let Some(other) = core.get_player_by_uid(i) {
                    if other.get_name().to_lowercase().contains(&query) {
                        // Found a match.
                        // Check the worlds.
                        if other.get_world() != player.get_world() {
                            other_world = String::from(other.get_world());
                        }

                        found_position = Some(other.get_transform().clone());

                        break;
                    }
                }
            }
        }

        // This action is done outside the loop, to prevent deadlocks.
        if let Some(pos) = found_position {
            // In a different world, try to join.
            if other_world != player.get_world() {
                player.try_join_world(core, &other_world);
            }

            player.update_transform(pos);
        } else {
            player.send_message(&format!(
                "&7Couldn't find a player with name \"{}\".",
                query
            ));
        }
    }
}

fn worlds_command(core: &Core, player: &mut (dyn Player), message: &str, surpress: &mut bool) {
    // Event already handled.
    if *surpress {
        return;
    }

    if message.to_lowercase().starts_with("/worlds") {
        player.send_message("&6Due to API Limitations, we can only display");
        player.send_message(&format!(
            "&6the number of worlds loaded: &8{}",
            core.get_world_count()
        ));

        *surpress = true;
    }
}

fn players_command(core: &Core, player: &mut (dyn Player), message: &str, surpress: &mut bool) {
    // Event already handled.
    if *surpress {
        return;
    }

    if message.to_lowercase().starts_with("/players") {
        player.send_message("&6Players Online:");

        let user_self = String::from(player.get_display_name()); // Cannot borrow at the same time and call.
        player.send_message(&user_self);

        // TODO: find better way to iterate.
        for i in 0..128 {
            if i != player.get_uid() {
                if let Some(other) = core.get_player_by_uid(i) {
                    player.send_message(other.get_display_name());
                }
            }
        }

        *surpress = true;
    }
}

// Calls the chat hook. Register your own event systems down below.
pub fn on_message(
    core: &Core,
    player: &mut WriteGuard<usize, Box<dyn Player + Send + Sync>>,
    message: String,
) -> bool {
    let mut surpress = false;

    help_command(core, player.as_mut(), &message, &mut surpress);
    main_command(core, player.as_mut(), &message, &mut surpress);
    join_command(core, player.as_mut(), &message, &mut surpress);
    tp_command(core, player.as_mut(), &message, &mut surpress);
    worlds_command(core, player.as_mut(), &message, &mut surpress);
    players_command(core, player.as_mut(), &message, &mut surpress);

    surpress
}

//! The `UI` struct is responsible for handling the rendering of the game's user interface.
//! It interacts with the terminal using the `crossterm` library for clear and dynamic updates.
//!
//! The UI includes:
//! - Rendering the game map with various tiles, the player's position, and enemies.
//! - Displaying the player's inventory below the map.
//! - Showing control instructions.
//! - Providing space for messages like death or victory notifications.
//!
//! ### Fields
//! - `last_rendered_height`:
//!   Tracks the last height where content was rendered in the terminal.
//!   This is useful for positioning messages correctly.
//!
//! ### Methods
//! - `UI::new()`:
//!   Creates a new `UI` instance with default values.
//!
//! - `UI::render()`:
//!   Renders the game map, player position, inventory, and controls within the terminal. Clears the terminal first
//!   to ensure seamless and clean updates.
//!
//! - `UI::show_death_message()`:
//!   Displays a "You died" message in the terminal.
//!
//! - `UI::show_game_clear_message()`:
//!   Displays a "Game clear" message in the terminal.
//!
//! - `UI::show_message()`:
//!   Displays a custom message at the correct screen position while temporarily pausing
//!   the program to make the message visible for the user.
//!
//! ### Notes
//! This struct heavily relies on the ANSI escape codes managed by the `crossterm` library to dynamically update the terminal output.
//!
//! The rendering logic primarily focuses on minimal terminal updates and user feedback during the game.

use crate::classes::level::Level;
use crate::classes::player::Player;
use crate::classes::types::{ItemType, Position, TileType};
use crossterm::{
    cursor::{Hide, MoveTo},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::io::{stdout, Write};

pub struct UI {
    last_rendered_height: u16, // Track the last rendered height to know where to put messages
}

impl Default for UI {
    fn default() -> Self {
        Self::new()
    }
}

impl UI {
    pub fn new() -> Self {
        Self {
            last_rendered_height: 0,
        }
    }

    pub fn render(&mut self, level: &Level, player: &Player) {
        let mut stdout = stdout();
        stdout.execute(Clear(ClearType::All)).unwrap();
        stdout.execute(MoveTo(0, 0)).unwrap();
        stdout.execute(Hide).unwrap();

        // Build the complete frame in a string first
        let mut frame = String::new();
        for (row, row_tiles) in level.map.iter().enumerate() {
            let mut line = String::new();
            for (col, tile) in row_tiles.iter().enumerate() {
                let pos = Position {
                    row: row as i16,
                    col: col as i16,
                };

                let char = if pos == player.pos {
                    "🥷"
                } else if level.enemies.contains(&pos) {
                    "🧌"
                } else {
                    match tile {
                        TileType::Empty => "・",
                        TileType::Wall => "🌲",
                        TileType::Bamboo => "🎋",
                        TileType::Mountain => "⛰️\u{200B}",
                        TileType::Water => "🟦",
                        TileType::Volcano => "🌋",
                        TileType::Lava => "🟧",
                        TileType::SnowMountain => "🗻",
                        TileType::Goal => "🏯",
                        TileType::Axe => "🪓",
                        TileType::WoodLog => "🪵",
                        TileType::Canoe => "🛶",
                        TileType::Sword => "🗡\u{200B}",
                        TileType::Key => "🗝️\u{200B}",
                        TileType::Door => "🚪",
                        TileType::DoorOpen => "⛩️\u{200B}",
                        TileType::Cottage => "🏚️\u{200B}",
                        TileType::Tomb => "🪦",
                        TileType::Rock => "🪨",
                        TileType::Bomb => "💣",
                        TileType::Hook => "🪝",
                        TileType::HookStart => "⚓",
                        TileType::HookEnd => "⚓",
                        TileType::Link => "🔗",
                        TileType::CrystalA => "🔮",
                        TileType::CrystalB => "🔮",
                        TileType::CrystalC => "🔮",
                        TileType::FlameA => "🔥",
                        TileType::FlameB => "🔥",
                        TileType::FlameC => "🔥",
                        TileType::Alembic => "⚗️\u{200B}",
                        TileType::WindChime => "🎐",
                        TileType::DragonSword => "⚔️\u{200B}",
                        TileType::Lantern => "🏮",
                        TileType::Oni => "👹",
                        TileType::Boss => "🎎",
                        TileType::Princess => "🧝‍♀️",
                    }
                };
                line.push_str(char);
            }
            line.push_str("\r\n");
            frame.push_str(&line);
        }

        // Add inventory display below the map
        frame.push_str(" 🎒 Inventory: ");
        if player.inventory.is_empty() {
            frame.push_str("Empty");
        } else {
            for item in &player.inventory {
                let item_char = match item {
                    ItemType::Axe => "🪓",
                    ItemType::Sword => "🗡\u{200B}",
                    ItemType::Key => "🗝️\u{200B}",
                    ItemType::Bomb => "💣",
                    ItemType::Hook => "🪝",
                    ItemType::WindChime => "🎐",
                    ItemType::DragonSword => "⚔️",
                };
                frame.push_str(item_char);
                frame.push(' ');
            }
        }

        frame.push_str("\r\n");
        frame.push_str(" wasd: Move | q: Quit");

        // Reserve a line for messages
        frame.push_str("\r\n");

        // Calculate total lines
        self.last_rendered_height = level.map.len() as u16 + 3; // map + inventory + controls + empty

        // Write the complete frame at once
        // print!("{}", frame);
        write!(stdout, "{}", frame).unwrap();
        stdout.flush().unwrap();
    }

    pub fn show_death_message(&self) {
        self.show_message("    ☠️  You died ☠️");
    }

    pub fn show_game_clear_message(&self) {
        self.show_message("   🎊 Game clear 🎊");
    }

    pub fn show_message(&self, message: &str) {
        let mut stdout = stdout();

        // Use the stored last_rendered_height to position the message
        stdout
            .execute(MoveTo(0, self.last_rendered_height))
            .unwrap();
        stdout.execute(Clear(ClearType::CurrentLine)).unwrap();

        // Write the message
        writeln!(stdout, "{}", message).unwrap();
        stdout.flush().unwrap();

        // Sleep to show message
        std::thread::sleep(std::time::Duration::from_secs_f32(1.2));

        // Clear the line again
        stdout
            .execute(MoveTo(0, self.last_rendered_height))
            .unwrap();
        stdout.execute(Clear(ClearType::CurrentLine)).unwrap();
        stdout.flush().unwrap();
    }
}

use crossterm::{
    cursor::{Hide, MoveTo},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::io::{stdout, Write};
use crate::classes::types::{Position, TileType, ItemType};
use crate::classes::level::Level;
use crate::classes::player::Player;

pub struct UI {}

impl UI {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, level: &Level, player: &Player) {
        let mut stdout = stdout();
        stdout.execute(Clear(ClearType::All)).unwrap();
        stdout.execute(MoveTo(0, 0)).unwrap();
        stdout.execute(Hide).unwrap();

        // Build the complete frame in a string first
        let mut frame = String::new();
        for (row, row_tiles) in level.map.iter().enumerate() {
            let mut line = String::new();
            for (col, tile) in row_tiles.iter().enumerate() {
                let pos = Position { row: row as i16, col: col as i16 };

                let char = if pos == player.pos {
                    "🥷"
                } else if level.enemies.contains(&pos) {
                    "🧌"
                } else {
                    match tile {
                        TileType::Empty => "・",
                        TileType::Wall => "🌲",
                        TileType::Bamboo => "🎋",
                        TileType::Water => "🟦",
                        TileType::Goal => "🏯",
                        TileType::Axe => "🪓",
                        TileType::WoodLog => "🪵",
                        TileType::Canoe => "🛶",
                        // TileType::Hook => "🪝",
                        // TileType::Door => "🚪",
                    }
                };
                line.push_str(char);
            }
            line.push_str("\r\n");
            frame.push_str(&line);
        }

        // Add inventory display below the map
        frame.push_str("\r\n   🎒 Inventory: ");
        if player.inventory.is_empty() {
            frame.push_str("Empty");
        } else {
            for item in &player.inventory {
                let item_char = match item {
                    ItemType::Axe => "🪓",
                };
                frame.push_str(item_char);
                frame.push_str(" ");
            }
        }

        frame.push_str("\r\n");

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

    fn show_message(&self, message: &str) {
        println!("{}", message);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
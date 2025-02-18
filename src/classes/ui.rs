use crossterm::{
    cursor::{Hide, MoveTo},
    terminal::{Clear, ClearType},
    style::Print,
    queue,
    ExecutableCommand, QueueableCommand
};
use std::io::{stdout, Write};
use crate::classes::types::{Position, TileType};
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
                let pos = Position { row: row as u16, col: col as u16 };

                let char = if pos == player.pos {
                    "🥷"
                } else if level.enemies.contains(&pos) {
                    "🧌"
                } else {
                    match tile {
                        TileType::Empty => "・",
                        TileType::Wall => "🌲",
                        TileType::Bamboo => "🎋",
                        TileType::Goal => "🏯",
                        // TileType::Hook => "🪝",
                        // TileType::Bridge => "🪵",
                        // TileType::Door => "🚪",
                    }
                };
                line.push_str(char);
            }
            line.push_str("\r\n");
            frame.push_str(&line);
            // frame.push('\n'); // go to next line
            // frame.push('\r'); // carriage return
        }

        // Write the complete frame at once
        // print!("{}", frame);
        write!(stdout, "{}", frame).unwrap();
        stdout.flush().unwrap();
    }

    // pub fn render(&self, level: &Level, player: &Player) {
    //     let mut stdout = stdout();
    //     // stdout.execute(Clear(ClearType::All)).unwrap();
    //     // stdout.execute(MoveTo(0, 0)).unwrap();
    //     // stdout.execute(Hide).unwrap();
    //     // stdout.execute(Print("Styled text here.")).unwrap();
    //
    //     queue!(stdout, Clear(ClearType::All));
    //     queue!(stdout, MoveTo(0, 0));
    //     queue!(stdout, Hide);
    //     // queue!(stdout, Print("Styled text here."));
    //
    //     // print!("a");
    //
    //     for (row, row_tiles) in level.map.iter().enumerate() {
    //         for (col, tile) in row_tiles.iter().enumerate() {
    //             let pos = Position { row: row as u16, col: col as u16 };
    //
    //             // print!("{},{}", row, col);
    //             queue!(stdout, MoveTo(pos.col, pos.row));
    //             // stdout.execute(MoveTo(pos.col, pos.row)).unwrap();
    //
    //             if pos == player.pos {
    //                 // print!("🥷");
    //                 queue!(stdout, Print("🥷"));
    //             } else if level.enemies.contains(&pos) {
    //                 // print!("🧌");
    //                 queue!(stdout, Print("🧌"));
    //             } else {
    //                 match tile {
    //                     TileType::Empty => queue!(stdout, Print("・")),
    //                     TileType::Wall => queue!(stdout, Print("🌲")),
    //                     TileType::Bamboo => queue!(stdout, Print("🎋")),
    //                     TileType::Goal => queue!(stdout, Print("🏯")),
    //                     // TileType::Empty => print!("・"),
    //                     // TileType::Wall => print!("🌲"),
    //                     // TileType::Bamboo => print!("🎋"),
    //                     // TileType::Goal => print!("🏯"),
    //                     // TileType::Hook => print!("🪝"),
    //                     // TileType::Bridge => print!("🪵"),
    //                     // TileType::Door => print!("🚪"),
    //                 }
    //             }
    //         }
    //         // print!("\r\n");
    //         // println!();
    //     }
    //
    //     stdout.flush().unwrap();
    // }

    pub fn show_death_message(&self) {
        println!("    ☠️  You died ☠️");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
//!
//! The `Level` struct represents a game level and its associated properties,
//! including a map layout, enemy positions, player starting position, and the
//! map's dimensions. Levels are loaded from files with a specific format, allowing
//! the dynamic creation of game environments.
//!
//! # Fields
//! - `map`: A 2D vector of `TileType` that represents the physical layout of the level.
//! - `enemies`: A vector of `Position` structs representing the positions of enemies in the level.
//! - `player_start`: A `Position` indicating the starting position of the player.
//! - `map_size`: A tuple `(u8, u8)` that specifies the number of rows and columns in the level map.
//!
//! # Methods
//!
//! ## `load`
//! Loads a level from a file specified by its `level_number`.
//!
//! - File Path: The method looks for a file named `maps/level_<level_number>.txt`.
//! - Parses each character in the file to create the level map and determines special positions, such as enemies and the player's start.
//!
//! Returns:
//! - `Some(Level)`: If the file is successfully read and parsed.
//! - `None`: If the file is not found or there is a read error.
//!
//! ## `set_tile`
//! Sets a specified tile in the map to a new `TileType`.
//!
//! Arguments:
//! - `pos` (`&Position`): The position in the map to be updated.
//! - `tile_type` (`TileType`): The new tile type to set at the given position.
//!
//! Notes:
//! - The method ensures the provided position is within the map boundaries.
//!
//! ## `get_tile`
//! Retrieves the `TileType` at a specified position in the map.
//!
//! Arguments:
//! - `pos` (`&Position`): The position to look up.
//!
//! Returns:
//! - `Some(TileType)`: If the position is within the map boundaries.
//! - `None`: If the position is out of bounds.
//!
//! # Example Tile Mapping Description
//! The `load` method interprets characters in a `.txt` file to initialize the level,
//! with each character corresponding to a `TileType`.
//! For example:
//! - `'t'`: Wall
//! - `'p'`: Player starting position
//! - `'e'`: Enemy
//! - `'s'`: Empty space
//!
//! Additional characters map to their respective `TileType` as defined in the `match` block.
//!

use crate::classes::types::{Position, TileType};
use std::fs;

pub struct Level {
    pub map: Vec<Vec<TileType>>,
    pub enemies: Vec<Position>,
    pub player_start: Position,
    pub map_size: (u8, u8),
    // pub goal: Position, // Did not need goal pos returned, as we have tile map
}

impl Level {
    pub fn load(level_number: usize) -> Option<Self> {
        let filename = format!("maps/level_{}.txt", level_number);
        if let Ok(contents) = fs::read_to_string(&filename) {
            let mut map = Vec::new();
            let mut enemies = Vec::new();
            let mut player_start = Position { row: 0, col: 0 };
            // let mut goal = Position { row: 0, col: 0 };

            for (row, line) in contents.lines().enumerate() {
                let mut map_row = Vec::new();
                for (col, c) in line.chars().enumerate() {
                    match c {
                        't' => map_row.push(TileType::Wall),
                        'b' => map_row.push(TileType::Bamboo),
                        'm' => map_row.push(TileType::Mountain),
                        'v' => map_row.push(TileType::Volcano),
                        'n' => map_row.push(TileType::SnowMountain),
                        'h' => map_row.push(TileType::Cottage),
                        's' => map_row.push(TileType::Empty),
                        'w' => map_row.push(TileType::Water),
                        'z' => map_row.push(TileType::Lava),
                        'a' => map_row.push(TileType::Axe),
                        'l' => map_row.push(TileType::WoodLog),
                        'c' => map_row.push(TileType::Canoe),
                        '+' => map_row.push(TileType::Sword),
                        'k' => map_row.push(TileType::Key),
                        'd' => map_row.push(TileType::Door),
                        'D' => map_row.push(TileType::DoorOpen),
                        'r' => map_row.push(TileType::Rock),
                        '@' => map_row.push(TileType::Bomb),
                        'j' => map_row.push(TileType::Hook),
                        '1' => map_row.push(TileType::HookStart),
                        '2' => map_row.push(TileType::HookEnd),
                        '-' => map_row.push(TileType::Link),
                        'A' => map_row.push(TileType::CrystalA),
                        'B' => map_row.push(TileType::CrystalB),
                        'C' => map_row.push(TileType::CrystalC),
                        '3' => map_row.push(TileType::FlameA),
                        '4' => map_row.push(TileType::FlameB),
                        '5' => map_row.push(TileType::FlameC),
                        'i' => map_row.push(TileType::WindChime),
                        'x' => map_row.push(TileType::DragonSword),
                        'O' => map_row.push(TileType::Lantern),
                        'o' => map_row.push(TileType::Oni),
                        '0' => map_row.push(TileType::Boss),
                        '$' => map_row.push(TileType::Princess),
                        'p' => {
                            map_row.push(TileType::Empty);
                            player_start = Position {
                                row: row as i16,
                                col: col as i16,
                            };
                        }
                        'e' => {
                            map_row.push(TileType::Empty);
                            enemies.push(Position {
                                row: row as i16,
                                col: col as i16,
                            });
                        }
                        'g' => {
                            map_row.push(TileType::Goal);
                        }
                        _ => map_row.push(TileType::Empty),
                    }
                }
                map.push(map_row);
            }

            let map_size = (map.len() as u8, map[0].len() as u8);

            Some(Level {
                map,
                enemies,
                player_start,
                map_size,
            })
        } else {
            None
        }
    }

    pub fn set_tile(&mut self, pos: &Position, tile_type: TileType) {
        if pos.row >= 0
            && pos.row < self.map_size.0 as i16
            && pos.col >= 0
            && pos.col < self.map_size.1 as i16
        {
            self.map[pos.row as usize][pos.col as usize] = tile_type;
        }
    }

    pub fn get_tile(&self, pos: &Position) -> Option<TileType> {
        if pos.row >= 0
            && pos.row < self.map_size.0 as i16
            && pos.col >= 0
            && pos.col < self.map_size.1 as i16
        {
            Some(self.map[pos.row as usize][pos.col as usize])
        } else {
            None
        }
    }
}

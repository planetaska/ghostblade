use std::fs;
use crate::classes::types::{Position, TileType};

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
                        's' => map_row.push(TileType::Empty),
                        'w' => map_row.push(TileType::Water),
                        'p' => {
                            map_row.push(TileType::Empty);
                            player_start = Position { row: row as i16, col: col as i16 };
                        },
                        'e' => {
                            map_row.push(TileType::Empty);
                            enemies.push(Position { row: row as i16, col: col as i16 });
                        },
                        'd' => {
                            map_row.push(TileType::Goal);
                            // goal = Position { row: row as i16, col: col as i16 };
                        },
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
                // goal,
            })
        } else {
            None
        }
    }
}
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
                        'r' => map_row.push(TileType::Rock),
                        'f' => map_row.push(TileType::Hook),
                        '1' => map_row.push(TileType::HookStart),
                        '2' => map_row.push(TileType::HookEnd),
                        '-' => map_row.push(TileType::Link),
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

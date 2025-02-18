use crate::classes::types::{TileType, CollisionType, Position};
use crate::classes::player::Player;
use crate::classes::level::Level;
use crate::classes::ui::UI;
use rand::Rng;

pub struct Game {
    current_level: usize,
    max_levels: usize,
    level: Level,
    ui: UI,
}

impl Game {
    pub fn new() -> Self {
        let current_level = 1;
        let max_levels = 2;
        let level = Level::load(current_level).expect("Failed to load first level");
        let ui = UI::new();

        Self {
            current_level,
            max_levels,
            level,
            ui,
        }
    }

    pub fn init_player(&self) -> Player {
        let mut player = Player::new();
        player.reset_position(self.level.player_start);
        player
    }

    pub fn check_collision(&self, pos: &Position) -> CollisionType {
        // Check bounds
        if pos.row < 0 || pos.row >= self.level.map.len() as u16 ||
            pos.col < 0 || pos.col >= self.level.map[0].len() as u16 {
            return CollisionType::OutOfBounds;
        }

        // Check static obstacles
        match self.level.map[pos.row as usize][pos.col as usize] {
            TileType::Wall | TileType::Bamboo => return CollisionType::Wall,
            TileType::Goal => return CollisionType::Goal,
            _ => {}
        }

        // Check enemies
        if self.level.enemies.contains(pos) {
            return CollisionType::Enemy;
        }

        CollisionType::None
    }

    pub fn update_enemies(&mut self) {
        let mut rng = rand::rng();
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        // Extract enemies into a temporary variable to avoid borrowing issues
        let mut enemies = std::mem::take(&mut self.level.enemies);

        for enemy in &mut enemies {
            if rng.random_bool(0.8) {
                let (dy, dx) = directions[rng.random_range(0..4)];
                let new_pos = Position {
                    // should fix this!
                    row: (enemy.row as i16 + dy) as u16,
                    col: (enemy.col as i16 + dx) as u16,
                };

                if self.check_collision(&new_pos) == CollisionType::None {
                    *enemy = new_pos;
                }
            }
        }
    }

    pub fn advance_level(&mut self) -> bool {
        self.current_level += 1;
        if self.current_level <= self.max_levels {
            if let Some(new_level) = Level::load(self.current_level) {
                self.level = new_level;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn get_player_start(&self) -> Position {
        self.level.player_start
    }

    pub fn handle_player_death(&self) {
        self.ui.show_death_message();
    }

    pub fn render(&self, player: &Player) {
        self.ui.render(&self.level, player);
    }
}
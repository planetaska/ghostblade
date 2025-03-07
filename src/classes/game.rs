use crate::classes::level::Level;
use crate::classes::player::Player;
use crate::classes::types::{CollisionType, ItemType, Position, TileType};
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
        let max_levels = 4;
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
        if pos.row < 0
            || pos.row >= self.level.map_size.0 as i16
            || pos.col < 0
            || pos.col >= self.level.map_size.1 as i16
        {
            return CollisionType::OutOfBounds;
        }

        // Check static obstacles
        match self.level.map[pos.row as usize][pos.col as usize] {
            TileType::Wall | TileType::Bamboo | TileType::Water => return CollisionType::Wall,
            TileType::Goal => return CollisionType::Goal,
            TileType::Axe | TileType::Sword => return CollisionType::Item,
            TileType::WoodLog => return CollisionType::WoodLog,
            _ => {}
        }

        // Check enemies
        if self.level.enemies.contains(pos) {
            return CollisionType::Enemy;
        }

        CollisionType::None
    }

    pub fn handle_interaction(&mut self, player: &mut Player) {
        // If there's a pending move, check for interactions
        if let Some(new_pos) = player.get_pending_move() {
            match self.check_collision(&new_pos) {
                CollisionType::Item => {
                    // Get the tile at the new position
                    let tile_type = self.level.map[new_pos.row as usize][new_pos.col as usize];

                    // Handle item pickup based on tile type
                    match tile_type {
                        TileType::Axe => {
                            player.add_item(ItemType::Axe);
                            // Remove the axe from the map
                            self.level.map[new_pos.row as usize][new_pos.col as usize] =
                                TileType::Empty;
                            // Allow movement to this tile
                            player.commit_move();
                        }
                        TileType::Sword => {
                            player.add_item(ItemType::Sword);
                            // Remove the sword from the map
                            self.level.map[new_pos.row as usize][new_pos.col as usize] =
                                TileType::Empty;
                            // Allow movement to this tile
                            player.commit_move();
                        }
                        _ => {}
                    }
                }
                CollisionType::WoodLog => {
                    // Check if player has axe
                    if player.has_item(ItemType::Axe) {
                        // Find the water tile to the right of the log
                        let water_pos = Position {
                            row: new_pos.row,
                            col: new_pos.col + 1,
                        };

                        // Check if the tile to the right is water
                        if let Some(tile) = self.level.get_tile(&water_pos) {
                            if tile == TileType::Water {
                                // Convert the water tile to a canoe
                                self.level.set_tile(&water_pos, TileType::Canoe);
                                // Clear the wood log by converting it to empty space
                                self.level.set_tile(&new_pos, TileType::Empty);
                                // Remove the axe from inventory (it's consumed by use)
                                // self.remove_item_from_player(player, ItemType::Axe);
                                player.remove_item(ItemType::Axe);
                                // Player can't move onto the wood log
                                player.cancel_move();
                            }
                        }
                    } else {
                        // Can't interact with wood log without axe
                        player.cancel_move();
                    }
                }
                CollisionType::Enemy => {
                    // Check if player has sword
                    if player.has_item(ItemType::Sword) {
                        // Get enemy position
                        let enemy_pos = new_pos;

                        // Remove enemy at this position
                        self.remove_enemy(&enemy_pos);

                        // Consume the sword (one-time use)
                        player.remove_item(ItemType::Sword);

                        // Allow player to move to the enemy position
                        player.commit_move();
                    } else {
                        // Without sword, player dies on enemy collision
                        self.handle_player_death();
                        player.reset_position(self.get_player_start());
                    }
                }
                _ => {}
            }
        }
    }

    pub fn remove_enemy(&mut self, pos: &Position) {
        self.level.enemies.retain(|enemy| enemy != pos);
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
                    row: enemy.row + dy,
                    col: enemy.col + dx,
                };

                if self.check_collision(&new_pos) == CollisionType::None {
                    *enemy = new_pos;
                }
            }
        }

        self.level.enemies = enemies;
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

    pub fn handle_game_clear(&self) {
        self.ui.show_game_clear_message();
    }

    pub fn render(&self, player: &Player) {
        self.ui.render(&self.level, player);
    }
}

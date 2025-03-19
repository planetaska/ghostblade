use crate::classes::level::Level;
use crate::classes::player::Player;
use crate::classes::types::{CollisionType, InteractiveType, BlockingType, ItemType, Position, TileType};
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
        let current_level = 7;
        let max_levels = 7;
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
            return CollisionType::Blocking(BlockingType::Wall);
        }

        // Check static obstacles
        match self.level.map[pos.row as usize][pos.col as usize] {
            TileType::Wall => return CollisionType::Blocking(BlockingType::Wall),
            TileType::Bamboo => return CollisionType::Blocking(BlockingType::Bamboo),
            TileType::Mountain => return CollisionType::Blocking(BlockingType::Mountain),
            TileType::Water => return CollisionType::Blocking(BlockingType::Water),
            TileType::Volcano => return CollisionType::Blocking(BlockingType::Volcano),
            TileType::Lava => return CollisionType::Blocking(BlockingType::Lava),
            TileType::Goal => return CollisionType::Goal,
            TileType::Axe => return CollisionType::Interactive(InteractiveType::Item(ItemType::Axe)),
            TileType::Sword => return CollisionType::Interactive(InteractiveType::Item(ItemType::Sword)),
            TileType::Key => return CollisionType::Interactive(InteractiveType::Item(ItemType::Key)),
            TileType::Hook => return CollisionType::Interactive(InteractiveType::Item(ItemType::Hook)),
            TileType::WoodLog => return CollisionType::Interactive(InteractiveType::WoodLog),
            TileType::Door => return CollisionType::Interactive(InteractiveType::Door),
            TileType::Cottage => return CollisionType::Interactive(InteractiveType::Cottage),
            TileType::Rock => return CollisionType::Interactive(InteractiveType::Rock),
            TileType::HookStart => return CollisionType::Interactive(InteractiveType::HookStart),
            _ => {}
        }

        // Check enemies
        if self.level.enemies.contains(pos) {
            return CollisionType::Interactive(InteractiveType::Enemy);
        }

        CollisionType::None
    }

    pub fn handle_interaction(&mut self, player: &mut Player) {
        // If there's a pending move, check for interactions
        if let Some(new_pos) = player.get_pending_move() {
            match self.check_collision(&new_pos) {
                CollisionType::Interactive(interactive_type) => {
                    match interactive_type {
                        InteractiveType::Item(item_type) => {
                            self.handle_item_pickup(player, &new_pos, item_type);
                        },
                        InteractiveType::WoodLog => {
                            self.handle_wood_log(player, &new_pos);
                        },
                        InteractiveType::Door => {
                            self.handle_door(player, &new_pos);
                        },
                        InteractiveType::Cottage => {
                            self.handle_cottage(player, &new_pos);
                        },
                        InteractiveType::Rock => {
                            self.handle_rock(player, &new_pos);
                        },
                        InteractiveType::HookStart => {
                            self.handle_hook_start(player, &new_pos);
                        },
                        InteractiveType::Enemy => {
                            self.handle_enemy(player, &new_pos);
                        },
                    }
                },
                _ => {}
            }
        }
    }

    fn handle_item_pickup(&mut self, player: &mut Player, pos: &Position, item_type: ItemType) {
        // Add the item to player's inventory
        player.add_item(item_type);

        // Remove the item from the map
        self.level.map[pos.row as usize][pos.col as usize] = TileType::Empty;

        // Allow movement to this tile
        player.commit_move();
    }

    fn handle_wood_log(&mut self, player: &mut Player, pos: &Position) {
        if player.has_item(ItemType::Axe) {
            // Check for water to the right
            let water_pos = Position {
                row: pos.row,
                col: pos.col + 1,
            };

            let is_water_to_right = self.level.get_tile(&water_pos) == Some(TileType::Water);

            if is_water_to_right {
                // Water to the right - create canoe
                self.level.set_tile(&water_pos, TileType::Canoe);
                self.level.set_tile(pos, TileType::Empty);
                player.remove_item(ItemType::Axe);
                player.cancel_move();
            } else {
                // No water - just remove log
                self.level.set_tile(pos, TileType::Empty);
                player.remove_item(ItemType::Axe);
                player.commit_move();
            }
        } else {
            player.cancel_move();
        }
    }

    fn handle_door(&mut self, player: &mut Player, pos: &Position) {
        if player.has_item(ItemType::Key) {
            self.level.set_tile(pos, TileType::DoorOpen);
            player.remove_item(ItemType::Key);
            player.cancel_move();
        } else {
            player.cancel_move();
        }
    }

    fn handle_cottage(&mut self, player: &mut Player, pos: &Position) {
        self.level.set_tile(pos, TileType::Tomb);
        player.add_item(ItemType::Bomb);
        player.cancel_move();
    }

    fn handle_rock(&mut self, player: &mut Player, pos: &Position) {
        if player.has_item(ItemType::Bomb) {
            self.level.set_tile(pos, TileType::Empty);
            player.remove_item(ItemType::Bomb);
            player.cancel_move();
        } else {
            player.cancel_move();
        }
    }

    fn handle_hook_start(&mut self, player: &mut Player, pos: &Position) {
        if player.has_item(ItemType::Hook) {
            // Find the HookEnd tile
            let mut hook_end = None;
            for row in 0..self.level.map_size.0 as i16 {
                for col in 0..self.level.map_size.1 as i16 {
                    let current_pos = Position { row, col };
                    if let Some(tile) = self.level.get_tile(&current_pos) {
                        if tile == TileType::HookEnd {
                            hook_end = Some(current_pos);
                            break;
                        }
                    }
                }
                if hook_end.is_some() {
                    break;
                }
            }

            if let Some(end_pos) = hook_end {
                // Change the start position to Link
                self.level.set_tile(pos, TileType::Link);

                // Check if we can make a straight line path
                if pos.row == end_pos.row {
                    // Horizontal path
                    let start_col = pos.col.min(end_pos.col);
                    let end_col = pos.col.max(end_pos.col);

                    for col in start_col..=end_col {
                        let current = Position { row: pos.row, col };
                        self.level.set_tile(&current, TileType::Link);
                    }
                } else if pos.col == end_pos.col {
                    // Vertical path
                    let start_row = pos.row.min(end_pos.row);
                    let end_row = pos.row.max(end_pos.row);

                    for row in start_row..=end_row {
                        let current = Position { row, col: pos.col };
                        self.level.set_tile(&current, TileType::Link);
                    }
                }
            }

            player.remove_item(ItemType::Hook);
            player.cancel_move();
        } else {
            player.cancel_move();
        }
    }

    fn handle_enemy(&mut self, player: &mut Player, pos: &Position) {
        if player.has_item(ItemType::Sword) {
            // Remove enemy at this position
            self.remove_enemy(pos);
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

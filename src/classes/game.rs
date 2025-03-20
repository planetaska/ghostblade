//! The `Game` structure represents the main logic and state of the game.
//!
//! # Fields
//!
//! - `current_level` (`usize`): The current level the player is on.
//! - `max_levels` (`usize`): The total number of levels in the game.
//! - `level` (`Level`): The instance of the current level, managing the map and environmental data.
//! - `ui` (`UI`): The User Interface handler for rendering the game state.
//! - `boss_health` (`u8`): The current health points of the boss entity.
//!
//! # Methods
//!
//! - `default`: Provides a default implementation for the game.
//! - `new`: Creates a new instance of the `Game` initialized with the first level, UI, and default settings.
//! - `init_player`: Initializes the player for the current level, positioning them in the starting location.
//! - `check_collision`: Checks if the given position collides with any object or boundary in the game and returns a `CollisionType`.
//! - `handle_interaction`: Handles player interactions based on their pending movement and interactions with interactive objects like items, doors, or enemies.
//! - `find_tile`: A helper method to find the position of a specific tile type in the map.
//! - `has_any_tile`: Checks if any of the specified tile types exist on the current level map.
//!
//! # Usage
//!
//! ```rust,ignore
//! // Create a new game instance
//! let mut game = Game::new();
//!
//! // Initialize the player
//! let mut player = game.init_player();
//!
//! // Check for interactions while navigating the game
//! game.handle_interaction(&mut player);
//! ```

use crate::classes::level::Level;
use crate::classes::player::Player;
use crate::classes::types::{
    BlockingType, CollisionType, InteractiveType, ItemType, Position, TileType,
};
use crate::classes::ui::UI;
use rand::Rng;

pub struct Game {
    current_level: usize,
    max_levels: usize,
    pub level: Level,
    ui: UI,
    boss_health: u8,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Self {
        let current_level = 1;
        let max_levels = 10;
        let level = Level::load(current_level).expect("Failed to load first level");
        let ui = UI::new();

        Self {
            current_level,
            max_levels,
            level,
            ui,
            boss_health: 3,
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
            TileType::SnowMountain => return CollisionType::Blocking(BlockingType::SnowMountain),
            TileType::FlameA => return CollisionType::Blocking(BlockingType::FlameA),
            TileType::FlameB => return CollisionType::Blocking(BlockingType::FlameB),
            TileType::FlameC => return CollisionType::Blocking(BlockingType::FlameC),
            TileType::Lantern => return CollisionType::Blocking(BlockingType::Lantern),
            TileType::Goal => return CollisionType::Goal,
            TileType::Princess => return CollisionType::Princess,
            TileType::Axe => {
                return CollisionType::Interactive(InteractiveType::Item(ItemType::Axe))
            }
            TileType::Sword => {
                return CollisionType::Interactive(InteractiveType::Item(ItemType::Sword))
            }
            TileType::Key => {
                return CollisionType::Interactive(InteractiveType::Item(ItemType::Key))
            }
            TileType::Bomb => {
                return CollisionType::Interactive(InteractiveType::Item(ItemType::Bomb))
            }
            TileType::Hook => {
                return CollisionType::Interactive(InteractiveType::Item(ItemType::Hook))
            }
            TileType::WindChime => {
                return CollisionType::Interactive(InteractiveType::Item(ItemType::WindChime))
            }
            TileType::DragonSword => {
                return CollisionType::Interactive(InteractiveType::Item(ItemType::DragonSword))
            }
            TileType::WoodLog => return CollisionType::Interactive(InteractiveType::WoodLog),
            TileType::Door => return CollisionType::Interactive(InteractiveType::Door),
            TileType::Cottage => return CollisionType::Interactive(InteractiveType::Cottage),
            TileType::Rock => return CollisionType::Interactive(InteractiveType::Rock),
            TileType::HookStart => return CollisionType::Interactive(InteractiveType::HookStart),
            TileType::CrystalA => return CollisionType::Interactive(InteractiveType::CrystalA),
            TileType::CrystalB => return CollisionType::Interactive(InteractiveType::CrystalB),
            TileType::CrystalC => return CollisionType::Interactive(InteractiveType::CrystalC),
            TileType::Oni => return CollisionType::Interactive(InteractiveType::Oni),
            TileType::Boss => return CollisionType::Interactive(InteractiveType::Boss),
            _ => {}
        }

        if self.level.enemies.contains(pos) {
            return CollisionType::Interactive(InteractiveType::Enemy);
        }

        CollisionType::None
    }

    pub fn handle_interaction(&mut self, player: &mut Player) {
        if let Some(new_pos) = player.get_pending_move() {
            if let CollisionType::Interactive(interactive_type) = self.check_collision(&new_pos) {
                match interactive_type {
                    InteractiveType::Item(item_type) => {
                        self.handle_item_pickup(player, &new_pos, item_type);
                    }
                    InteractiveType::WoodLog => {
                        self.handle_wood_log(player, &new_pos);
                    }
                    InteractiveType::Door => {
                        self.handle_door(player, &new_pos);
                    }
                    InteractiveType::Cottage => {
                        self.handle_cottage(player, &new_pos);
                    }
                    InteractiveType::Rock => {
                        self.handle_rock(player, &new_pos);
                    }
                    InteractiveType::HookStart => {
                        self.handle_hook_start(player, &new_pos);
                    }
                    InteractiveType::CrystalA => {
                        self.handle_crystal(player, &new_pos);
                    }
                    InteractiveType::CrystalB => {
                        self.handle_crystal(player, &new_pos);
                    }
                    InteractiveType::CrystalC => {
                        self.handle_crystal(player, &new_pos);
                    }
                    InteractiveType::Enemy => {
                        self.handle_enemy(player, &new_pos);
                    }
                    InteractiveType::Oni => {
                        self.handle_oni(player, &new_pos);
                    }
                    InteractiveType::Boss => {
                        self.handle_boss(player, &new_pos);
                    }
                }
            }
        }
    }

    fn find_tile(&self, tile_type: TileType) -> Option<Position> {
        for row in 0..self.level.map_size.0 as usize {
            for col in 0..self.level.map_size.1 as usize {
                if self.level.map[row][col] == tile_type {
                    return Some(Position {
                        row: row as i16,
                        col: col as i16,
                    });
                }
            }
        }
        None
    }

    fn has_any_tile(&self, tile_types: &[TileType]) -> bool {
        for tile_type in tile_types {
            if self.find_tile(*tile_type).is_some() {
                return true;
            }
        }
        false
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
                self.ui.show_message("   You crafted a canoe ");
                player.cancel_move();
            } else {
                // No water - just remove log
                self.level.set_tile(pos, TileType::Empty);
                player.remove_item(ItemType::Axe);
                self.ui.show_message("   You chopped the log ");
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
            self.ui.show_message("   You opened the door ");
            player.cancel_move();
        } else {
            player.cancel_move();
        }
    }

    fn handle_cottage(&mut self, player: &mut Player, pos: &Position) {
        self.level.set_tile(pos, TileType::Tomb);
        player.add_item(ItemType::Bomb);
        self.ui.show_message("   You found a bomb ");
        player.cancel_move();
    }

    fn handle_rock(&mut self, player: &mut Player, pos: &Position) {
        if player.has_item(ItemType::Bomb) {
            self.level.set_tile(pos, TileType::Empty);
            player.remove_item(ItemType::Bomb);
            self.ui.show_message("  💥 The rock crumbles to dust 💥");
            player.cancel_move();
        } else {
            player.cancel_move();
        }
    }

    fn handle_hook_start(&mut self, player: &mut Player, pos: &Position) {
        if player.has_item(ItemType::Hook) {
            let hook_end = self.find_tile(TileType::HookEnd);

            if let Some(end_pos) = hook_end {
                self.level.set_tile(pos, TileType::Link);

                if pos.row == end_pos.row {
                    let start_col = pos.col.min(end_pos.col);
                    let end_col = pos.col.max(end_pos.col);

                    for col in start_col..=end_col {
                        let current = Position { row: pos.row, col };
                        self.level.set_tile(&current, TileType::Link);
                    }
                } else if pos.col == end_pos.col {
                    let start_row = pos.row.min(end_pos.row);
                    let end_row = pos.row.max(end_pos.row);

                    for row in start_row..=end_row {
                        let current = Position { row, col: pos.col };
                        self.level.set_tile(&current, TileType::Link);
                    }
                }
            }

            player.remove_item(ItemType::Hook);
            self.ui.show_message("   You hooked the link ");
            player.cancel_move();
        } else {
            player.cancel_move();
        }
    }

    fn handle_crystal(&mut self, player: &mut Player, pos: &Position) {
        let crystal_type = self.level.map[pos.row as usize][pos.col as usize];

        let flame_type = match crystal_type {
            TileType::CrystalA => TileType::FlameA,
            TileType::CrystalB => TileType::FlameB,
            TileType::CrystalC => TileType::FlameC,
            _ => return,
        };

        if let Some(flame_pos) = self.find_tile(flame_type) {
            self.level.set_tile(&flame_pos, TileType::Empty);
            self.ui.show_message("   The flame vanishes ");
        }

        self.level.set_tile(pos, TileType::Alembic);

        let flame_types = [TileType::FlameA, TileType::FlameB, TileType::FlameC];
        let all_flames_removed = !self.has_any_tile(&flame_types);

        if all_flames_removed {
            player.add_item(ItemType::WindChime);
            self.ui.show_message("   🎐 You received a Wind Chime! 🎐");
        }

        player.cancel_move();
    }

    fn handle_oni(&mut self, player: &mut Player, pos: &Position) {
        if player.has_item(ItemType::WindChime) {
            self.level.set_tile(pos, TileType::Empty);
            player.remove_item(ItemType::WindChime);
            self.ui.show_message("   The wind chime cleanses the air ");
            player.add_item(ItemType::DragonSword);
            self.ui.show_message("   You found a Dragon Sword ");
            player.commit_move();
        } else {
            self.handle_player_death();
            player.reset_position(self.get_player_start());
        }
    }

    fn handle_boss(&mut self, player: &mut Player, pos: &Position) {
        if player.has_item(ItemType::DragonSword) {
            self.ui.show_message("   ⚔️\u{200B} Clash! ⚔️\u{200B}");

            if self.boss_health > 0 {
                self.boss_health -= 1;

                if self.boss_health > 0 {
                    self.ui
                        .show_message("   You are pushed away by the strong impact...");
                    self.ui
                        .show_message(&format!("   Boss health: {}/3", self.boss_health));
                }

                if self.boss_health == 0 {
                    self.level.set_tile(pos, TileType::Empty);
                    self.ui.show_message("  💥 Boss defeated! 💥");
                    player.commit_move();
                    return;
                }
            }

            player.reset_position(self.get_player_start());
        } else {
            self.handle_player_death();
            player.reset_position(self.get_player_start());
        }
    }

    fn handle_enemy(&mut self, player: &mut Player, pos: &Position) {
        if player.has_item(ItemType::Sword) {
            self.remove_enemy(pos);
            player.remove_item(ItemType::Sword);
            self.ui
                .show_message("   You slayed an enemy, a small victory ");
            player.commit_move();
        } else {
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

    pub fn render(&mut self, player: &Player) {
        self.ui.render(&self.level, player);
    }
}

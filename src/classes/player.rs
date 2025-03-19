//! Module containing the `Player` struct and its associated methods.
//!
//! The `Player` struct is used to represent a player, including their position,
//! inventory, and movement functionality within a game. The struct provides
//! methods to manipulate the player's state, such as moving, managing an inventory,
//! and resetting or committing movement.
//!
//! ## Structs
//!
//! - `Player`: The main struct representing a player in the game.
//!
//! ## Methods
//!
//! ### `Player`
//!
//! - `new`: Creates a new `Player` instance with default values.
//! - `reset_position`: Resets the player's position to a specified value.
//! - `move_up`: Sets a pending move to one row up.
//! - `move_down`: Sets a pending move to one row down.
//! - `move_left`: Sets a pending move to one column left.
//! - `move_right`: Sets a pending move to one column right.
//! - `get_pending_move`: Retrieves the pending move, if any.
//! - `commit_move`: Commits the pending move and updates the player's position.
//! - `cancel_move`: Cancels the pending move.
//! - `add_item`: Adds an item to the player's inventory.
//! - `has_item`: Checks if the player has a specific item in their inventory.
//! - `remove_item`: Removes an item from the player's inventory if it exists.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use crate::classes::types::{ItemType, Position};
//! use crate::classes::Player;
//!
//! // Create a new player
//! let mut player = Player::new();
//!
//! // Reset the player's position
//! player.reset_position(Position { row: 1, col: 1 });
//!
//! // Move the player up and commit the move
//! player.move_up();
//! player.commit_move();
//!
//! // Add an item to the player's inventory
//! player.add_item(ItemType::Sword);
//!
//! // Check if the player has a specific item
//! if player.has_item(ItemType::Sword) {
//!     println!("The player has a sword!");
//! }
//!
//! // Remove the item from the player's inventory
//! player.remove_item(ItemType::Sword);
//! ```

use crate::classes::types::{ItemType, Position};

pub struct Player {
    pub pos: Position,
    pending_move: Option<Position>,
    pub inventory: Vec<ItemType>,
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos: Position { row: 0, col: 0 },
            pending_move: None,
            inventory: Vec::new(),
        }
    }

    pub fn reset_position(&mut self, pos: Position) {
        self.pos = pos;
        self.pending_move = None;
    }

    pub fn move_up(&mut self) {
        self.pending_move = Some(Position {
            row: self.pos.row - 1,
            col: self.pos.col,
        });
    }

    pub fn move_down(&mut self) {
        self.pending_move = Some(Position {
            row: self.pos.row + 1,
            col: self.pos.col,
        });
    }

    pub fn move_left(&mut self) {
        self.pending_move = Some(Position {
            row: self.pos.row,
            col: self.pos.col - 1,
        });
    }

    pub fn move_right(&mut self) {
        self.pending_move = Some(Position {
            row: self.pos.row,
            col: self.pos.col + 1,
        });
    }

    pub fn get_pending_move(&self) -> Option<Position> {
        self.pending_move
    }

    pub fn commit_move(&mut self) {
        if let Some(new_pos) = self.pending_move {
            self.pos = new_pos;
        }
        self.pending_move = None;
    }

    pub fn cancel_move(&mut self) {
        self.pending_move = None;
    }

    pub fn add_item(&mut self, item: ItemType) {
        self.inventory.push(item);
    }

    pub fn has_item(&self, item: ItemType) -> bool {
        self.inventory.contains(&item)
    }

    pub fn remove_item(&mut self, item: ItemType) {
        if let Some(index) = self.inventory.iter().position(|&i| i == item) {
            self.inventory.remove(index);
        }
    }
}

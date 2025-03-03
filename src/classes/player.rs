use crate::classes::types::{Position, ItemType};

pub struct Player {
    pub pos: Position,
    pending_move: Option<Position>,
    pub inventory: Vec<ItemType>,
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
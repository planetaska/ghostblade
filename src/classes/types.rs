#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub row: u16,
    pub col: u16,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CollisionType {
    None,
    Wall,
    Enemy,
    Goal,
    OutOfBounds,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
    Empty,
    Wall,
    Bamboo,
    Goal,
    // Hook,
    // Bridge,
    // Door,
}
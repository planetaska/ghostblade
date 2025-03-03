#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub row: i16,
    pub col: i16,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CollisionType {
    None,
    Wall,
    Enemy,
    Item,
    Goal,
    OutOfBounds,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
    Empty,
    Wall,
    Bamboo,
    Water,
    Axe,
    WoodLog,
    Canoe,
    Goal,
    // Hook,
    // Door,
}
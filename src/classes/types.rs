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
    WoodLog,
    Door,
    Goal,
    OutOfBounds,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
    Empty,
    Wall,
    Bamboo,
    Mountain,
    Water,
    Axe,
    WoodLog,
    Canoe,
    Sword,
    Key,
    Door,
    DoorOpen,
    Goal,
    // Hook,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ItemType {
    Axe,
    Sword,
    Key
}

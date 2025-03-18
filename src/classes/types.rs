#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub row: i16,
    pub col: i16,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CollisionType {
    None,
    Interactive(InteractiveType),
    Blocking,
    Goal,
}

// Specific types of interactive elements
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InteractiveType {
    Item(ItemType),
    WoodLog,
    Door,
    Cottage,
    Rock,
    Enemy,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ItemType {
    Axe,
    Sword,
    Key,
    Bomb
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
    Cottage,
    Tomb,
    Bomb,
    Rock,
    Goal,
    // Hook,
}

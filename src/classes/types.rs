#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub row: i16,
    pub col: i16,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CollisionType {
    None,
    Interactive(InteractiveType),
    Blocking(BlockingType),
    Goal,
    Princess,
}

// Specific types of interactive elements
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InteractiveType {
    Item(ItemType),
    WoodLog,
    Door,
    Cottage,
    Rock,
    HookStart,
    CrystalA,
    CrystalB,
    CrystalC,
    Enemy,
    Oni,
    Boss,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlockingType {
    Wall,
    Bamboo,
    Mountain,
    Water,
    Volcano,
    Lava,
    SnowMountain,
    FlameA,
    FlameB,
    FlameC,
    Lantern,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ItemType {
    Axe,
    Sword,
    Key,
    Bomb,
    Hook,
    WindChime,
    DragonSword,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
    Empty,
    Wall,
    Bamboo,
    Mountain,
    Water,
    Volcano,
    Lava,
    SnowMountain,
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
    Hook,
    HookStart,
    HookEnd,
    Link,
    CrystalA,
    CrystalB,
    CrystalC,
    FlameA,
    FlameB,
    FlameC,
    Alembic,
    WindChime,
    Lantern,
    DragonSword,
    Oni,
    Boss,
    Princess,
}

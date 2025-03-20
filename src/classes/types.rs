//! # Game Map and Collision System
//!
//! This module defines the fundamental components necessary for handling
//! map positions, tile interactions, collisions, and various item types
//! in the game. It provides structured enumerations and reusable data
//! structures for implementing gameplay mechanics.
//!
//! ## Key Components
//!
//! ### `Position`
//! A struct used to represent a 2D position on the map.
//!
//! Fields:
//! - `row` (i16): The row index of the position.
//! - `col` (i16): The column index of the position.
//!
//! Example:
//! ```rust,ignore
//! let position = Position { row: 5, col: 3 };
//! println!("{:?}", position); // Outputs: Position { row: 5, col: 3 }
//! ```
//!
//! ### `CollisionType`
//! Represents the type of collision for a tile in the map.
//!
//! Variants:
//! - `None`: No collision.
//! - `Interactive(InteractiveType)`: Represents an interactive element.
//! - `Blocking(BlockingType)`: Represents a blocking element.
//! - `Goal`: Represents the goal position.
//! - `Princess`: Represents the princess tile.
//!
//! Example:
//! ```rust,ignore
//! let collision = CollisionType::Goal;
//! match collision {
//!     CollisionType::Goal => println!("You reached the goal!"),
//!     _ => println!("Keep moving!"),
//! }
//! ```
//!
//! ### `InteractiveType`
//! Represents the types of interactive objects in the game.
//!
//! Variants:
//! - `Item(ItemType)`: An item, such as a key or sword.
//! - `WoodLog`, `Door`, `Cottage`, `Rock`, etc.: Various interactive elements specific to gameplay.
//!
//! Example:
//! ```rust,ignore
//! let interactive = InteractiveType::Door;
//! match interactive {
//!     InteractiveType::Door => println!("You found a door."),
//!     _ => println!("This is something else."),
//! }
//! ```
//!
//! ### `BlockingType`
//! Represents objects or elements that block the player's movement.
//!
//! Variants:
//! - `Wall`, `Bamboo`, `Mountain`, etc.: Various blocking terrain types.
//!
//! Example:
//! ```rust,ignore
//! let block = BlockingType::Wall;
//! match block {
//!     BlockingType::Wall => println!("You cannot pass the wall."),
//!     _ => println!("This is not blocking."),
//! }
//! ```
//!
//! ### `ItemType`
//! Represents collectible items or tools.
//!
//! Variants:
//! - `Axe`, `Sword`, `Key`, `Bomb`, etc.: Various collectible or usable items.
//!
//! Example:
//! ```rust,ignore
//! let item = ItemType::Key;
//! match item {
//!     ItemType::Key => println!("You picked up a key."),
//!     _ => println!("You found another item."),
//! }
//! ```
//!
//! ### `TileType`
//! Represents different tile types in the game map.
//!
//! Variants:
//! - Examples include `Empty`, `Wall`, `Goal`, `Princess`, `Cottage`, etc.
//!
//! Example:
//! ```rust,ignore
//! let tile = TileType::Goal;
//! match tile {
//!     TileType::Goal => println!("You've stepped on the goal tile!"),
//!     _ => println!("This is a different tile."),
//! }
//! ```

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

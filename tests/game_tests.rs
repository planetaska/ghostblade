use ghostblade::classes::game::Game;
use ghostblade::classes::level::Level;
use ghostblade::classes::player::Player;
use ghostblade::classes::types::{CollisionType, ItemType, Position, TileType};

#[test]
fn test_game_initialization() {
    let game = Game::new();
    let player = game.init_player();

    assert_eq!(player.pos, game.get_player_start());
    assert!(player.inventory.is_empty());
}

#[test]
fn test_player_movement() {
    let mut player = Player::new();
    player.reset_position(Position { row: 5, col: 5 });

    player.move_up();
    assert_eq!(player.get_pending_move(), Some(Position { row: 4, col: 5 }));
    player.commit_move();
    assert_eq!(player.pos, Position { row: 4, col: 5 });

    player.move_right();
    assert_eq!(player.get_pending_move(), Some(Position { row: 4, col: 6 }));
    player.commit_move();
    assert_eq!(player.pos, Position { row: 4, col: 6 });

    player.move_down();
    assert_eq!(player.get_pending_move(), Some(Position { row: 5, col: 6 }));
    player.cancel_move();
    assert_eq!(player.pos, Position { row: 4, col: 6 });

    player.move_left();
    assert_eq!(player.get_pending_move(), Some(Position { row: 4, col: 5 }));
    player.commit_move();
    assert_eq!(player.pos, Position { row: 4, col: 5 });
}

#[test]
fn test_collision_detection() {
    let mut map = vec![vec![TileType::Empty; 5]; 5];
    map[1][1] = TileType::Wall;
    map[1][2] = TileType::Goal;
    map[1][3] = TileType::Key;

    let level = Level {
        map,
        enemies: vec![Position { row: 3, col: 3 }],
        player_start: Position { row: 0, col: 0 },
        map_size: (5, 5),
    };

    let mut game = Game::new();
    let _ = std::mem::replace(&mut game.level, level);

    let empty_pos = Position { row: 0, col: 1 };
    assert_eq!(game.check_collision(&empty_pos), CollisionType::None);

    let wall_pos = Position { row: 1, col: 1 };
    match game.check_collision(&wall_pos) {
        CollisionType::Blocking(_) => (), // Expected
        _ => panic!("Expected blocking collision with wall"),
    }

    let goal_pos = Position { row: 1, col: 2 };
    assert_eq!(game.check_collision(&goal_pos), CollisionType::Goal);

    let key_pos = Position { row: 1, col: 3 };
    match game.check_collision(&key_pos) {
        CollisionType::Interactive(_) => (), // Expected
        _ => panic!("Expected interactive collision with key"),
    }

    let enemy_pos = Position { row: 3, col: 3 };
    match game.check_collision(&enemy_pos) {
        CollisionType::Interactive(_) => (), // Expected
        _ => panic!("Expected interactive collision with enemy"),
    }

    let out_of_bounds = Position { row: -1, col: 0 };
    match game.check_collision(&out_of_bounds) {
        CollisionType::Blocking(_) => (), // Expected
        _ => panic!("Expected blocking collision with out of bounds"),
    }
}

#[test]
fn test_inventory_management() {
    let mut player = Player::new();

    player.add_item(ItemType::Key);
    assert!(player.has_item(ItemType::Key));
    assert!(!player.has_item(ItemType::Sword));

    player.add_item(ItemType::Sword);
    assert!(player.has_item(ItemType::Key));
    assert!(player.has_item(ItemType::Sword));

    player.remove_item(ItemType::Key);
    assert!(!player.has_item(ItemType::Key));
    assert!(player.has_item(ItemType::Sword));

    player.remove_item(ItemType::Sword);
    assert!(!player.has_item(ItemType::Sword));
    assert!(player.inventory.is_empty());
}

#[test]
fn test_level_loading() {
    if let Some(level) = Level::load(1) {
        assert!(!level.map.is_empty());

        assert!(level.player_start.row >= 0 && level.player_start.row < level.map_size.0 as i16);
        assert!(level.player_start.col >= 0 && level.player_start.col < level.map_size.1 as i16);
    } else {
        panic!("Failed to load level 1");
    }
}

#[test]
fn test_level_tile_operations() {
    let mut level = Level::load(1).expect("Failed to load level");

    let test_pos = Position { row: 2, col: 2 };
    let original_tile = level.get_tile(&test_pos).unwrap();

    level.set_tile(&test_pos, TileType::Door);
    assert_eq!(level.get_tile(&test_pos).unwrap(), TileType::Door);

    level.set_tile(&test_pos, original_tile);
    assert_eq!(level.get_tile(&test_pos).unwrap(), original_tile);

    let invalid_pos = Position { row: -1, col: -1 };
    assert!(level.get_tile(&invalid_pos).is_none());
}

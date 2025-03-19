// Crate crossterm: Cross-platform Terminal Manipulation Library
// https://docs.rs/crossterm/latest/crossterm/
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io;
use std::time::{Duration, Instant};

mod classes;

use classes::game::Game;
use classes::types::CollisionType;

fn main() -> io::Result<()> {
    // Initialize game state
    let mut game = Game::new();
    let mut player = game.init_player();

    // Setup terminal
    enable_raw_mode()?;

    // Game loop timing
    // let frame_duration = Duration::from_millis(16); // ~60 FPS
    let frame_duration = Duration::from_millis(160); // ~6 FPS
    let enemy_move_interval = Duration::from_millis(500);
    let mut last_enemy_move = Instant::now();

    // Main game loop
    'game_loop: loop {
        let frame_start = Instant::now();

        // Handle input
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('q') | KeyCode::Esc => break 'game_loop,
                    KeyCode::Char('w') | KeyCode::Up => player.move_up(),
                    KeyCode::Char('s') | KeyCode::Down => player.move_down(),
                    KeyCode::Char('a') | KeyCode::Left => player.move_left(),
                    KeyCode::Char('d') | KeyCode::Right => player.move_right(),
                    _ => {}
                }
            }
        }

        // Update game state
        if last_enemy_move.elapsed() >= enemy_move_interval {
            game.update_enemies();
            last_enemy_move = Instant::now();
        }

        // Process player movement and collisions
        if let Some(new_pos) = player.get_pending_move() {
            match game.check_collision(&new_pos) {
                CollisionType::None => player.commit_move(),
                CollisionType::Goal => {
                    if game.advance_level() {
                        player.reset_position(game.get_player_start());
                    }
                }
                CollisionType::Princess => {
                    game.handle_game_clear();
                    break 'game_loop;
                }
                CollisionType::Interactive(_) => {
                    // Handle all interactive elements with a single case
                    game.handle_interaction(&mut player);
                }
                CollisionType::Blocking(_) => {
                    // All blocking elements (walls, mountains, water, etc.)
                    player.cancel_move();
                }
            }
        }

        // Render
        game.render(&player);

        // Frame timing
        let elapsed = frame_start.elapsed();
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }
    }

    // Cleanup
    disable_raw_mode()?;
    Ok(())
}

//! The main entry point of the game.
//!
//! This function initializes the game state, sets up the terminal
//! in raw mode, and runs the main game loop. The game loop handles
//! user input, updates the game state, processes collisions, and renders
//! the game at a fixed frame rate. It also handles level progression,
//! interactions, and game completion.
//!
//! # Returns
//!
//! An `io::Result<()>` indicating whether the program ran successfully
//! or not. Any I/O-related errors during terminal setup or event handling
//! are returned.
//!
//! # Controls
//! - `w` or `Arrow Up`: Move the player up.
//! - `s` or `Arrow Down`: Move the player down.
//! - `a` or `Arrow Left`: Move the player left.
//! - `d` or `Arrow Right`: Move the player right.
//! - `q` or `Escape`: Quit the game.
//!
//! # Cleanup
//! Before exiting, this function ensures that the terminal is restored
//! to its normal state by disabling raw mode.

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
    let mut game = Game::new();
    let mut player = game.init_player();

    enable_raw_mode()?;

    let fps = 10;
    let frame_duration = Duration::from_secs_f32(1.0 / fps as f32);
    let enemy_move_interval = Duration::from_millis(500);
    let mut last_enemy_move = Instant::now();

    'game_loop: loop {
        let frame_start = Instant::now();

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

        if last_enemy_move.elapsed() >= enemy_move_interval {
            game.update_enemies();
            last_enemy_move = Instant::now();
        }

        if let Some(new_pos) = player.get_pending_move() {
            match game.check_collision(&new_pos) {
                CollisionType::None => player.commit_move(),
                CollisionType::Goal => {
                    if game.advance_level() {
                        player.reset_position(game.get_player_start());
                    } else {
                        game.handle_game_clear();
                        break 'game_loop;
                    }
                }
                CollisionType::Princess => {
                    game.handle_game_clear();
                    break 'game_loop;
                }
                CollisionType::Interactive(_) => {
                    game.handle_interaction(&mut player);
                }
                CollisionType::Blocking(_) => {
                    player.cancel_move();
                }
            }
        }

        game.render(&player);

        let elapsed = frame_start.elapsed();
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }
    }

    disable_raw_mode()?;
    Ok(())
}

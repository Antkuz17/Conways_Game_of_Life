mod grid;
mod game_state;

use game_state::GameState;
use macroquad::prelude::*;

#[macroquad::main("Game of Life")]
async fn main() {
    let mut state = GameState::new(50, 50);
    
    loop {
        next_frame().await;
    }
}
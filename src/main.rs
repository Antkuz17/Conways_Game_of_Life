mod grid;
mod game_state;

use game_state::GameState;
use macroquad::prelude::*;

#[macroquad::main("Game of Life")]
async fn main() {
    let mut state = GameState::new(50, 50);

    // glider for initial population
    state.current_grid.set_cell(1, 2, true);
    state.current_grid.set_cell(2, 3, true);
    state.current_grid.set_cell(3, 1, true);
    state.current_grid.set_cell(3, 2, true);
    state.current_grid.set_cell(3, 3, true);

    loop {
        for row in 0..state.current_grid.get_height()
    {
        for col in 0..state.current_grid.get_width() 
        {
            if state.current_grid.get_cell(row, col).unwrap_or(false) {
                draw_rectangle(
                    (col * 16) as f32 + 1.0,
                    (row * 16) as f32 + 1.0,
                    16.0,
                    16.0,
                    WHITE       
                );
            }
            else {
                // Black fill
                draw_rectangle(
                    (col * 16) as f32 + 1.0,
                    (row * 16) as f32 + 1.0,
                    16.0,
                    16.0,
                    BLACK
                );
            }

        }
    }
        std::thread::sleep(std::time::Duration::from_millis(75));
        state.update();
        next_frame().await;
    }
}

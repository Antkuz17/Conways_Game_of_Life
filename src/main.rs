mod grid;
mod game_state;

use game_state::GameState;
use macroquad::prelude::*;

#[macroquad::main("Game of Life")]
async fn main() {
    let mut state = GameState::new(50, 50);

    load_glider_gun(&mut state);

    // Uncomment one of the following lines to load a different starting configuration
    // load_random(&mut state);
    // load_glider(&mut state);

    
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

        fn load_glider(state: &mut GameState) {
        state.current_grid.set_cell(1, 2, true);
        state.current_grid.set_cell(2, 3, true);
        state.current_grid.set_cell(3, 1, true);
        state.current_grid.set_cell(3, 2, true);
        state.current_grid.set_cell(3, 3, true);
    }

    fn load_glider_gun(state: &mut GameState) {
        state.current_grid.set_cell(5, 1, true);
        state.current_grid.set_cell(5, 2, true);
        state.current_grid.set_cell(6, 1, true);
        state.current_grid.set_cell(6, 2, true);
        state.current_grid.set_cell(5, 11, true);
        state.current_grid.set_cell(6, 11, true);
        state.current_grid.set_cell(7, 11, true);
        state.current_grid.set_cell(4, 12, true);
        state.current_grid.set_cell(8, 12, true);
        state.current_grid.set_cell(3, 13, true);
        state.current_grid.set_cell(9, 13, true);
        state.current_grid.set_cell(3, 14, true);
        state.current_grid.set_cell(9, 14, true);
        state.current_grid.set_cell(6, 15, true);
        state.current_grid.set_cell(4, 16, true);
        state.current_grid.set_cell(8, 16, true);
        state.current_grid.set_cell(5, 17, true);
        state.current_grid.set_cell(6, 17, true);
        state.current_grid.set_cell(7, 17, true);
        state.current_grid.set_cell(6, 18, true);
        state.current_grid.set_cell(3, 21, true);
        state.current_grid.set_cell(4, 21, true);
        state.current_grid.set_cell(5, 21, true);
        state.current_grid.set_cell(3, 22, true);
        state.current_grid.set_cell(4, 22, true);
        state.current_grid.set_cell(5, 22, true);
        state.current_grid.set_cell(2, 23, true);
        state.current_grid.set_cell(6, 23, true);
        state.current_grid.set_cell(1, 25, true);
        state.current_grid.set_cell(2, 25, true);
        state.current_grid.set_cell(6, 25, true);
        state.current_grid.set_cell(7, 25, true);
        state.current_grid.set_cell(3, 35, true);
        state.current_grid.set_cell(4, 35, true);
        state.current_grid.set_cell(3, 36, true);
        state.current_grid.set_cell(4, 36, true);
    }

    fn load_random(state: &mut GameState) {
        for row in 0..state.current_grid.get_height() {
            for col in 0..state.current_grid.get_width() {
                if rand::gen_range(0, 100) < 15 {
                    state.current_grid.set_cell(row, col, true);
                }
            }
        }
    }
}

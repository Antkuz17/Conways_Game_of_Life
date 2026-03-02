mod grid;
mod game_state;

use game_state::GameState;
use macroquad::prelude::*;

#[macroquad::main("Game of Life")]
async fn main() {
    let mut state = GameState::new(100, 150);

    // cribbage
    //load_rle(&mut state, "4b2o$4bo$b2obo10bo$bo2b2o9b3o$3bo2bo11bo$bob4o10b2o$obo$o2b4o12b3o$b2o3bo11bo3bo6b2o$3b2o5b3o4bo5bo4bobo$3bo5bo3bo4bo3bo5bo$bobo4bo5bo4b3o5b2o$b2o6bo3bo11bo3b2o$10b3o12b4o2bo$29bobo$13b2o10b4obo$13bo11bo2bo$14b3o9b2o2bo$16bo10bob2o$27bo$26b2o!", 20, 50);

    // Uncomment one of the following lines to load a different starting configuration
    // load_random(&mut state);
    // load_glider(&mut state);
    load_glider_gun(&mut state);



    loop {
        for row in 0..state.current_grid.get_height()
    {
        for col in 0..state.current_grid.get_width() 
        {
            if state.current_grid.get_cell(row, col).unwrap_or(false) {
                draw_rectangle(
                    (col * 12) as f32 + 1.0,
                    (row * 12) as f32 + 1.0,
                    12.0,
                    12.0,
                    WHITE       
                );
            }
            else {
                // Black fill
                draw_rectangle(
                    (col * 12) as f32 + 1.0,
                    (row * 12) as f32 + 1.0,
                    12.0,
                    12.0,
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

    
    fn load_rle(state: &mut GameState, rle: &str, offset_row: usize, offset_col: usize) {
    let mut row = offset_row;
    let mut col = offset_col;
    let mut count_str = String::new();

    for ch in rle.chars() {
        match ch {
            '0'..='9' => count_str.push(ch),
            'b' => {
                let count = if count_str.is_empty() { 1 } else { count_str.parse::<usize>().unwrap() };
                col += count;
                count_str.clear();
            }
            'o' => {
                let count = if count_str.is_empty() { 1 } else { count_str.parse::<usize>().unwrap() };
                for i in 0..count {
                    state.current_grid.set_cell(row, col + i, true);
                }
                col += count;
                count_str.clear();
            }
            '$' => {
                let count = if count_str.is_empty() { 1 } else { count_str.parse::<usize>().unwrap() };
                row += count;
                col = offset_col;
                count_str.clear();
            }
            '!' => break,
            _ => {}
        }
    }
}

}

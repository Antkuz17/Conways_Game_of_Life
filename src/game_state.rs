use crate::grid::Grid;

pub struct GameState {
    current_grid: Grid,
    next_grid: Grid,
    current_generation: usize,
    current_population: usize,
}

impl GameState {
    pub fn new(width: usize, height: usize) -> Self{
        Self {
            current_grid: Grid::new(width, height),
            next_grid: Grid::new(width, height),
            current_generation: 0,
            current_population: 0,
        }
    }

    // Get generation number
    pub fn get_generation(&self) -> usize{
        self.current_generation
    }

    // Get current population
    pub fn get_population(&self) -> usize {
        self.current_population
    }

    // Get current population on the grid
    pub fn set_current_population(&mut self, population: usize) {
        self.current_population = population;
    }

    // Set current generation number
    pub fn set_current_generation(&mut self, generation: usize) {
        self.current_generation = generation;
    }
    /*
    Central logic function for the entire grid   
    The function will update the grid for the next generation
    */
    pub fn update(&mut self) {
        for row in 0..self.current_grid.get_height()
        {
            for col in 0..self.current_grid.get_width() 
            {
                // Get the number of live neighbors and the current cell status for the current cell
                let live_neighbors = self.current_grid.get_num_live(row, col);
                let current_cell = self.current_grid.get_cell(row, col).unwrap_or(false);

                // Apply the rules of the game to determine the status of the cell in the next generation

                // If the current cell is alive
                if current_cell {
                    // If the cell has 2-3 living partners it lives
                    if live_neighbors > 1 && live_neighbors < 4 {
                        self.next_grid.set_cell(row, col, true);
                        continue;
                    }
                    // Anything else and it dies
                    else{
                        self.next_grid.set_cell(row, col, false);
                    }
                }

                // If the current cell is dead
                else
                {
                    if live_neighbors == 3 {
                        self.next_grid.set_cell(row, col, true);
                    }
                }

            
            }
        }

        // Updating the current generation counter
        self.set_current_generation(self.get_generation() + 1);

        // Swap the current and next grid
        std::mem::swap(&mut self.current_grid, &mut self.next_grid);

        self.next_grid.clear();
    }
}
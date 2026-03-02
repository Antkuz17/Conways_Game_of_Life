use crate::grid::Grid;

pub struct GameState {
    CurrentGrid: Grid,
    NextGrid: Grid,
    CurrentGeneration: usize,
    CurrentPopulation: usize,
}

impl GameState {
    pub fn new(width: usize, height: usize) -> Self{
        Self {
            CurrentGrid: Grid::new(width, height),
            NextGrid: Grid::new(width, height),
            CurrentGeneration: 0,
            CurrentPopulation: 0,
        }
    }

    // Get generation number
    pub fn get_generation(&self) -> usize{
        self.CurrentGeneration
    }

    // Get current population
    pub fn get_population(&self) -> usize {
        self.CurrentPopulation
    }

    // Get current population on the grid
    pub fn set_current_population(&mut self, population: usize) {
        self.CurrentPopulation = population;
    }

    // Set current generation number
    pub fn set_current_generation(&mut self, generation: usize) {
        if generation < 0 {
            panic!("Generation number cannot be negative");
        }
        self.CurrentGeneration = generation;
    }
    /*
    Central logic function for the entire grid   
    The function will update the grid for the next generation
    */
    pub fn update(&mut self) {
        for row in 0..self.CurrentGrid.get_height()
        {
            for col in 0..self.CurrentGrid.get_width() 
            {
                // Get the number of live neighbors and the current cell status for the current cell
                let live_neighbors = self.CurrentGrid.get_num_live(row, col);
                let current_cell = self.CurrentGrid.get_cell(row, col).unwrap_or(false);

                // Apply the rules of the game to determine the status of the cell in the next generation

                // If the current cell is alive
                if (current_cell)
                {   
                    // If the cell has 2-3 living partners it lives
                    if live_neighbors > 1 && live_neighbors < 4 {
                        NextGrid.set_current_population(get)
                        continue;
                    }
                    // Anything else and it dies
                    else{
                        NextGrid.set_cell(row, col, false)
                    }
                }

                // If the current cell is dead
                else
                {
                    if live_neighbors == 3 {
                        NextGrid.set_cell(row, col, true);
                    }
                }

            
            }
        }

        // Updating the current generation counter
        set_current_generation(get_generation() + 1);

    }
}
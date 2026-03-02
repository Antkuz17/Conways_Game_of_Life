pub struct GameState {
    CurrentGrid: Grid,
    NextGrid: Grid,
    CurrentGeneration: usize,
    CurrentPopulation: usize,
}

impl GameState {
    pub fn new(width: usize, height: usize) -> Self{
        Self {
            CurrentGrid = Grid::new(width, height),
            NextGrid = Grid::new(width, height),
            CurrentGeneration = 0,
            CurrentPopulation = 0,
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
}
pub struct Grid {
    cells: Vec<bool>,
    width: usize,
    height: usize,
}

impl Grid {

    // "construtor" for the struct, no idea if this is proper convention but we ball
    pub fn new(width: usize, height:usize) -> Self {
        Self{
            width, 
            height,
            cells: vec![false; width * height]
        }
    }

    // Get function for cell status
    pub fn get_cell(&self, row: usize, col: usize) -> Option<bool>{
        if col >= self.width || row >= self.height {
            return None;
        }
        let index  = row * self.width + col;
        Some(self.cells[index])
    }

    // Set function for cell status
    pub fn set_cell(&mut self, row: usize, col: usize, value: bool) -> Option<()>{
        if col >= self.width || row >= self.height {
            return None;
        }
        let index  = row * self.width + col;
        self.cells[index] = value;
        Some(())
    }
    
    // Getter for width
    pub fn get_width(&self) -> usize {
        self.width
    }

    // Getter for height
    pub fn get_height(&self) -> usize {
        self.height
    }

    // Returns the number of live cells around a given cell
    pub fn get_num_live(&self, row: usize, col: usize) -> usize {
        
        // Counter for number of live cells
        let mut num_live: usize = 0;

        // Calculate the starting row and column for the nested loop, ensuring we don't go out of bounds
        // Avoids the issue of underflowing causing huge numbers when we try to subtract 1 from 0
        let row_start = row.saturating_sub(1);
        let col_start = col.saturating_sub(1);

        // Calculate the ending row and column for the nested loop, ensuring we don't go out of bounds
        let col_end = (col + 2).min(self.width);
        let row_end = (row + 2).min(self.height);

        // Nested for loop to iterate through the 8 cells around the cell of interest
        // Any cell outside grid is assumed dead
        for i in col_start..col_end{
            for z in row_start..row_end{

                // If the current cell is the cell of interest skip it
                if i == col && z == row {
                    continue;
                }

                // If the cell is alive, increment the counter
                if self.get_cell(z, i).unwrap_or(false) {
                    num_live += 1;
                }
            }
        }

        num_live
    }
}

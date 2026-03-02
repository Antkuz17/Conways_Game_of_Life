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
}
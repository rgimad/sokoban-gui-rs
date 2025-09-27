pub struct GameLevel {
    pub data: Vec<String>,
    pub width: usize,
    pub height: usize,
}

impl GameLevel {
    pub fn new() -> Self {
        Self {
            data: Vec::<String>::new(),
            width: 0,
            height: 0,
        }
    }

    pub fn from(level_data: Vec<String>) -> Self {
        let height = level_data.len();
        let width = if height > 0 {
            level_data[0].len()
        } else {
            0
        };
        // Validate that all rows have the same length
        for row in &level_data {
            if row.len() != width {
                panic!("Level rows must all have the same length");
            }
        }
        Self {
            data: level_data,
            width,
            height,
        }
    }

    pub fn get_cell(&self, rowcol: (usize, usize)) -> Option<char> {
        let row = rowcol.0;
        let col = rowcol.1;
        if col < self.width && row < self.height {
            self.data[row].chars().nth(col)
        } else {
            None
        }
    }

    pub fn set_cell(&mut self, rowcol: (usize, usize), cell: char) -> Result<(), &'static str> {
        let row = rowcol.0;
        let col = rowcol.1;
        if col >= self.width || row >= self.height {
            return Err("Coordinates out of bounds");
        }

        let mut row_chars: Vec<char> = self.data[row].chars().collect();
        row_chars[col] = cell;
        self.data[row] = row_chars.into_iter().collect();
        
        Ok(())
    }

    pub fn is_valid_position(&self, rowcol: (isize, isize)) -> bool {
        let row = rowcol.0;
        let col = rowcol.1;
        return col >= 0 && col < (self.width as isize) && row >= 0 && row < (self.height as isize);
    }
}

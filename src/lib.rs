use std::fmt;

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

fn clamp_sub(val: u32, max: u32) -> u32 {
    if val == 0 {
        max - 1
    } else {
        val - 1
    }
}

fn clamp_add(val: u32, max: u32) -> u32 {
    if val + 1 == max {
        0
    } else {
        val + 1
    }
}

impl Universe {
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    fn live_neighbour_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;

        let north = clamp_sub(row, self.height);
        let south = clamp_add(row, self.height);
        let west = clamp_sub(col, self.width);
        let east = clamp_add(col, self.width);

        count += self.cells[self.get_index(north, west)] as u8;
        count += self.cells[self.get_index(north, col)] as u8;
        count += self.cells[self.get_index(north, east)] as u8;
        count += self.cells[self.get_index(row, east)] as u8;
        count += self.cells[self.get_index(south, east)] as u8;
        count += self.cells[self.get_index(south, col)] as u8;
        count += self.cells[self.get_index(south, west)] as u8;
        count += self.cells[self.get_index(row, west)] as u8;

        count
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let i = self.get_index(row, col);
                let cell = self.cells[i];
                let live_neighbours = self.live_neighbour_count(row, col);

                let state = match (cell, live_neighbours) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (x, _) => x,
                };

                next[i] = state;
            }
        }

        self.cells = next;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn new(width: u32, height: u32) -> Universe {
        Universe {
            width,
            height,
            cells: (0..width * height).map(|_| Cell::Dead).collect(),
        }
    }

    pub fn bless_cell(&mut self, row: u32, col: u32) {
        let i = self.get_index(row, col);

        self.cells[i] = Cell::Alive;
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { ' ' } else { 'X' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
use std::convert::TryInto;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;

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

/// Wrap around subtraction of 1 from input, bound by max
fn clamp_sub(val: u32, max: u32) -> u32 {
    val.checked_sub(1).unwrap_or_else(|| max - 1)
}

/// Wrap around addition of 1 from input, bound by max
fn clamp_add(val: u32, max: u32) -> u32 {
    (val + 1) % max
}

fn wrapped_add_or_sub(val: u32, max: u32, offset: i8) -> u32 {
    match offset {
        -1 => clamp_sub(val, max),
        1 => clamp_add(val, max),
        _ => val,
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

        for lat_off in &[-1, 0, 1] {
            for lon_off in &[-1, 0, 1] {
                // don't count self
                if *lat_off == 0 && *lon_off == 0 {
                    continue;
                }
                let lat = wrapped_add_or_sub(row, self.height, *lat_off);
                let lon = wrapped_add_or_sub(col, self.width, *lon_off);
                count += self.cells[self.get_index(lat, lon)] as u8;
            }
        }

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

    pub fn from_file(file: &Path) -> Result<Universe, Box<dyn Error>> {
        let contents = fs::read_to_string(file.to_str().unwrap())?;

        let mut height = 0;
        let mut width = 0;

        let mut to_bless = Vec::new();

        for line in contents.lines() {
            if line.len() > width {
                width = line.len();
            }

            for (i, c) in line.chars().enumerate() {
                match c {
                    'X' | 'x' => to_bless.push((height, i)),
                    _ => {}
                }
            }

            height += 1;
        }

        let mut universe = Universe::new(width.try_into().unwrap(), height);

        for (row, col) in to_bless {
            universe.bless_cell(row, col as u32);
        }

        Ok(universe)
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { ' ' } else { '\u{2BC0}' };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clamp_sub() {
        assert_eq!(4, clamp_sub(0, 5));
        assert_eq!(1, clamp_sub(2, 5));
    }

    #[test]
    fn test_clamp_add() {
        assert_eq!(0, clamp_add(4, 5));
        assert_eq!(4, clamp_add(3, 5));
    }
}

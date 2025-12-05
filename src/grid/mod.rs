use std::ops::{Index, IndexMut};

mod resize;

use resize::Direction;

// Consider creating an Index struct with width and height fields

// store all the rows in one vec contiguously
//
// ------------------------> +width
// |
// |
// |
// |
// |
// |
// +
// heigth
pub struct Grid {
    pub width: u16,
    pub height: u16,
    cells: Vec<bool>,
    pub population: usize,
    pub generation: usize,

    // These flags is used to prevent resizing the terminal only from one direction
    // when the resize amount is an odd number.
    width_append_direction: Direction,
    height_append_direction: Direction,
    width_remove_direction: Direction,
    height_remove_direction: Direction,
}

impl Grid {
    pub fn new(width: u16, height: u16) -> Self {
        Grid {
            width,
            height,
            cells: vec![false; (width * height) as usize],
            // Todo: Check if population or generation aren't going out of bound
            population: 0,
            generation: 0,
            width_append_direction: Direction::Right,
            height_append_direction: Direction::Bottom,
            width_remove_direction: Direction::Right,
            height_remove_direction: Direction::Bottom,
        }
    }

    pub fn next_generation(&mut self) {
        let mut should_toggle_indices = Vec::<(u16, u16)>::new();

        for width in 0..self.width {
            for height in 0..self.height {
                let alive_neighbors_count = self.count_alive_neighbors((width, height));
                if self[(width, height)] {
                    if alive_neighbors_count < 2 || alive_neighbors_count > 3 {
                        should_toggle_indices.push((width, height));
                    }
                } else {
                    if alive_neighbors_count == 3 {
                        should_toggle_indices.push((width, height));
                    }
                }
            }
        }

        for index in should_toggle_indices {
            self.toggle_cell(index);
        }

        self.generation = self.generation + 1;
    }

    pub fn toggle_cell(&mut self, index: (u16, u16)) {
        self[index] = !self[index];

        if self[index] {
            self.population = self.population + 1;
        } else {
            self.population = self.population - 1;
        }
    }

    fn count_alive_neighbors(&self, (width, height): (u16, u16)) -> u8 {
        let mut alive_neighbors_count = 0_u8;
        let neighbor_offsets: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
        ];
        for neighbor_offset in neighbor_offsets {
            let neighbor_width = width as isize + neighbor_offset.0;
            let neighbor_height = height as isize + neighbor_offset.1;
            if neighbor_width < 0 || neighbor_height < 0 {
                continue;
            }
            // Todo: Check for the numeric type and make sure no overflow happens
            if let Some(is_alive) = self.get(neighbor_width as u16, neighbor_height as u16) {
                if *is_alive {
                    alive_neighbors_count = alive_neighbors_count + 1;
                }
            }
        }

        alive_neighbors_count
    }

    fn get(&self, width: u16, height: u16) -> Option<&bool> {
        // Return None if the index is out of bound
        if width >= self.width || height >= self.height {
            return None;
        }

        let width = width as usize;
        let height = height as usize;
        let grid_width = self.width as usize;
        // Todo: Check if the number doesn't go out of bound of usize
        self.cells.get(height * grid_width + width)
    }
}

impl Index<(u16, u16)> for Grid {
    type Output = bool;

    fn index(self: &Self, index: (u16, u16)) -> &Self::Output {
        let width = index.0 as usize;
        let height = index.1 as usize;
        let grid_width = self.width as usize;
        &self.cells[height * grid_width + width]
    }
}

impl IndexMut<(u16, u16)> for Grid {
    fn index_mut(self: &mut Self, index: (u16, u16)) -> &mut Self::Output {
        let width = index.0 as usize;
        let height = index.1 as usize;
        let grid_width = self.width as usize;
        &mut self.cells[height * grid_width + width]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alone_cells_should_die() {
        let mut grid = Grid::new(100, 100);
        grid.toggle_cell((0, 0));
        grid.toggle_cell((5, 5));
        grid.toggle_cell((99, 99));
        grid.toggle_cell((0, 50));
        grid.toggle_cell((50, 0));

        grid.next_generation();

        assert_eq!(grid.population, 0);
        assert!(
            grid.cells == vec![false; 10000],
            "The grid cells are not as expected!"
        );
    }

    #[test]
    fn cells_with_one_neighbor_should_die() {
        let mut grid = Grid::new(100, 100);
        let alive_cell_indices = [(0, 0), (0, 1), (5, 0), (5, 1), (98, 98), (99, 99)];
        for cell_index in alive_cell_indices {
            grid.toggle_cell(cell_index);
        }

        grid.next_generation();

        assert_eq!(grid.population, 0);
        for cell_index in alive_cell_indices {
            assert_eq!(grid[cell_index], false);
        }
    }

    // Blinker is a special pattern in game of life
    #[test]
    fn blinker() {
        let mut grid = Grid::new(100, 100);
        let alive_cell_indices = [(10, 10), (10, 11), (10, 12)];
        for cell_index in alive_cell_indices {
            grid.toggle_cell(cell_index);
        }

        grid.next_generation();
        assert_eq!(grid.population, 3);

        let alive_cell_indices = [(9, 11), (10, 11), (11, 11)];
        for cell_index in alive_cell_indices {
            assert_eq!(grid[cell_index], true);
        }
    }
}

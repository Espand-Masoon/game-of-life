use std::ops::{Index, IndexMut};

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
#[derive(Clone)]
pub struct Grid {
    pub width: u16,
    pub height: u16,
    cells: Vec<bool>,
    pub population: usize,
    pub generation: usize
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
        }
    }

    pub fn next_generation(&mut self) {
        let mut should_die_indices = Vec::<(u16, u16)>::new();
        let mut should_birth_indices = Vec::<(u16, u16)>::new();

        for width in 0..self.width {
            for height in 0..self.height {
                let mut alive_neighbors_count = 0_u8;
                let neighbor_offsets: [(isize, isize); 8] =
                [(-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1)];
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
                if self[(width, height)] {
                    if alive_neighbors_count < 2 || alive_neighbors_count > 3 {
                        should_die_indices.push((width, height));
                    }
                } else {
                    if alive_neighbors_count == 3 {
                        should_birth_indices.push((width, height));
                    }
                }
            }
        }

        // fn count_alive_neighbors(&self, width, height) -> u8 {
        //     
        // }

        self.population = self.population + should_birth_indices.len() - should_die_indices.len();

        for index in should_die_indices {
            self[index] = false;
        }

        for index in should_birth_indices {
            self[index] = true;
        }

        self.generation = self.generation + 1;
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

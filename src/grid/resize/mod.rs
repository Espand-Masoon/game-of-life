mod direction;

use std::{collections::VecDeque, vec};

use super::Grid;
pub use direction::Direction;

impl Grid {
    fn row_population(row: VecDeque<bool>) -> usize {
        let mut population = 0_usize;

        for cell in row {
            if cell {
                population = population + 1;
            }
        }

        population
    }
    // TODO: If the dimentions are lower than a limit, a message should be shown
    pub fn resize(&mut self, width: u16, height: u16) {
        // Todo: Check if anythinig goes out of bound in isize type
        let mut width_difference: isize = width as isize - self.width as isize;
        let mut height_difference: isize = height as isize - self.height as isize;
        let width_difference_half = width_difference.abs() / 2;
        let height_difference_half = height_difference.abs() / 2;

        // Decompose grid into rows
        // NOTE: The rows order is reversed after this conversion and should be reversed again at the end
        let mut rows: VecDeque<VecDeque<bool>> = VecDeque::new();
        for _ in 0..self.height {
            let row = self.cells.split_off(self.cells.len() - self.width as usize);
            rows.push_back(VecDeque::from(row));
        }

        // Adjust the grid to the new height
        if height_difference > 0 {
            let new_row: VecDeque<bool> = VecDeque::from(vec![false; self.width as usize]);
            // Add half of the new rows on the top of the grid and half on the bottom
            // TODO: check if we can get rid of clones and make it more efficient
            for _ in 0..height_difference_half {
                rows.push_front(new_row.clone());
                rows.push_back(new_row.clone());
            }

            // Add the extra row in case of the height_difference is an odd number
            if height_difference % 2 != 0 {
                if self.height_append_direction == Direction::Top {
                    rows.push_back(new_row.clone());
                } else if self.height_append_direction == Direction::Bottom {
                    rows.push_front(new_row.clone());
                }
                self.height_remove_direction = self.height_append_direction;
                self.height_append_direction.toggle();
            }
        }

        if height_difference < 0 {
            height_difference = height_difference * -1;
            for _ in 0..height_difference_half {
                // TODO: Check if you should propagate the error or unwrap is fine
                self.population = self.population
                    - Self::row_population(rows.pop_front().unwrap())
                    - Self::row_population(rows.pop_back().unwrap());
            }

            // TODO: Check if you should propagate the error or unwrap is fine
            if height_difference % 2 != 0 {
                if self.height_remove_direction == Direction::Top {
                    self.population =
                        self.population - Self::row_population(rows.pop_back().unwrap());
                }
                if self.height_remove_direction == Direction::Bottom {
                    self.population =
                        self.population - Self::row_population(rows.pop_front().unwrap());
                }
                self.height_append_direction = self.height_remove_direction;
                self.height_remove_direction.toggle();
            }
        }

        self.height = height;

        // Adjust the grid to the new width
        if width_difference > 0 {
            for row in rows.iter_mut() {
                for _ in 0..width_difference_half {
                    row.push_front(false);
                    row.push_back(false);
                }

                if width_difference % 2 != 0 {
                    if self.width_append_direction == Direction::Right {
                        row.push_back(false);
                    }
                    if self.width_append_direction == Direction::Left {
                        row.push_front(false);
                    }
                }
            }

            self.width_remove_direction = self.width_append_direction;
            self.width_append_direction.toggle();
        }

        if width_difference < 0 {
            for row in rows.iter_mut() {
                for _ in 0..width_difference_half {
                    self.population = self.population - row.pop_front().unwrap() as usize;
                    self.population = self.population - row.pop_back().unwrap() as usize;
                }
                if width_difference % 2 != 0 {
                    if self.width_remove_direction == Direction::Right {
                        self.population = self.population - row.pop_back().unwrap() as usize;
                    } else if self.width_remove_direction == Direction::Left {
                        self.population = self.population - row.pop_front().unwrap() as usize;
                    }
                }
            }

            if width_difference % 2 != 0 {
                self.width_append_direction = self.width_remove_direction;
                self.width_remove_direction.toggle();
            }
        }

        // TODO: Check if this operation is efficient
        self.cells = rows.into_iter().rev().flatten().collect();
        self.width = width;
    }
}

use rand::Rng; // ✅ Brings the `.gen()` method into scope

// ffsdg
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HealthState {
    Susceptible,
    Infected,
    Recovered,
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub state: HealthState,
}

pub struct Grid {
    pub grid_x: usize,
    pub grid_y: usize,
    pub cells: Vec<Cell>, // Flattened 2D grid
}

impl Grid {
    /// Creates a new grid of given dimensions.
    /// Most cells are `Susceptible`, with a small fraction randomly `Infected`.
    ///
    /// # Arguments
    /// * `width` - number of columns in the grid
    /// * `height` - number of rows in the grid
    /// * `infected_fraction` - fraction of the population to initialize as infected (e.g. 0.01 = 1%)
    pub fn init(grid_x: usize, grid_y: usize, infected_ratio: f64) -> Self {
        let size = grid_x * grid_y;
        let mut rng = rand::thread_rng(); // ✅ Get a random number generator

        let cells = (0..size)
            .map(|_| {
                let mut rng = rand::thread_rng(); // Get RNG
                let roll = rng.r#gen::<f64>(); // Generates a float between 0.0 and 1.0
                let state = if roll < infected_ratio {
                    HealthState::Infected
                } else {
                    HealthState::Susceptible
                };
                Cell { state }
            })
            .collect();

        Self {
            grid_x,
            grid_y,
            cells,
        }
    }
    /*
    /// Convert a 2D (x, y) position into a 1D index for accessing `cells`.
    /// Since we store the grid as a flat Vec<Cell>, this is how we simulate 2D access.
    pub fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.grid_x + x // row-major layout: rows first, then columns
    }

    /// Returns a list of valid neighboring coordinates for a given cell (x, y).
    /// This is an 8-connected neighborhood (including diagonals).
    pub fn get_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        // Allocate enough space for 8 neighbors up front (performance-friendly)
        let mut neighbors = Vec::with_capacity(8);

        // Loop over the 3x3 area around the (x, y) cell
        for dy in [-1, 0, 1] {
            for dx in [-1, 0, 1] {
                // Skip the center cell (itself)
                if dx == 0 && dy == 0 {
                    continue;
                }

                // Compute the neighbor's coordinates
                let nx = x as isize + dx;
                let ny = y as isize + dy;

                // Check if the neighbor is within the grid bounds
                if nx >= 0 && nx < self.grid_x as isize && ny >= 0 && ny < self.grid_y as isize {
                    // If it is, add it to the list (as a usize pair)
                    neighbors.push((nx as usize, ny as usize));
                }
            }
        }

        neighbors
    }
    */
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_case1() {
        let grid = Grid::init(10, 5, 0.0);
        assert_eq!(grid.grid_x, 10);
        assert_eq!(grid.grid_y, 5);
        assert_eq!(grid.cells.len(), 50);
    }
}
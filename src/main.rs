
mod utils {
    pub mod grid;
}
/*
mod maths;
mod simulation;

use crate::grid::{Grid, HealthState};
use crate::maths::SirParams;
use crate::simulation::step_grid;


fn main() {
    let mut grid = Grid::init(100, 100, 0.01); // 1% initially infected
    let params = SirParams { beta: 0.3, gamma: 0.1 };

    let mut time = 0.0;
    let dt = 1.0;

    while grid.count_infected() > 0 {
        step_grid(&mut grid, &params);
        time += dt;
        println!("Time: {:.1} | Infected: {}", time, grid.count_infected());
    }

    println!("Disease ran its course in {:.1} days", time);
}
*/

fn main() {
    println!("Compiles");
}

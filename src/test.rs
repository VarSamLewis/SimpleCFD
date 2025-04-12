#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_init_case1() {
        let grid = Grid::init(10, 5, 0.0);
        assert_eq!(grid.grid_x, 10);
        assert_eq!(grid.grid_y, 5);
        assert_eq!(grid.cells.len(), 50);
}

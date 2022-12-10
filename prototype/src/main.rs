pub mod grid;

fn main() {
    let g = crate::grid::grid::grid::Grid::new(10, 10);
    g.print_grid();
}

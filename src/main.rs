mod grid;
mod state;

fn main() {
    let g = state::Grid::new(10, 10);

    println!("{}", g);
}

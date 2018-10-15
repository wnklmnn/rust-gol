extern crate rust_gol;
fn main() {
    let mut field = rust_gol::GoLField::new(5000, 5000);
    field.set_cell_alive(1, 0); //010
    field.set_cell_alive(1, 1); //010
    field.set_cell_alive(1, 2); //010
    for _ in 0..10 {
        let start = std::time::Instant::now();
        for _ in 0..1 {
            let _ = field.calc_next_iteration(rust_gol::EdgeBehavior::DeadCells);
        }
        println!(
            "Time = {:#?}",
            std::time::Instant::now().duration_since(start)
        );
    }
}

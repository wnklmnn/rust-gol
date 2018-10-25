extern crate rust_gol;

fn main() {
    const COUNT: u32 = 100;
    let mut field = rust_gol::GoLField::new(500, 500);
    field.set_cell_alive(1, 0); //010
    field.set_cell_alive(1, 1); //010
    field.set_cell_alive(1, 2); //010
    let mut dur = std::time::Duration::new(0, 0);
    for _ in 0..COUNT {
        let start = std::time::Instant::now();
        eprintln!("{}", std::mem::size_of::<rust_gol::GoLField>());
        let _ = field.calc_next_iteration(&rust_gol::EdgeBehavior::Wrapping);
        let now = std::time::Instant::now();
        dur += now.duration_since(start);
        println!("time: {:#?}", now.duration_since(start));
    }
    println!("Durschnitt: {:#?}", dur / COUNT);
}

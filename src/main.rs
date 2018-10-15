extern crate rust_gol;
fn main() {
    let mut field = rust_gol::GoLField::new(10, 10);
    for h in 0..10 {
        for w in 0..10 {
            field.set_cell_alive(w, h);
        }
    }
    println!("{:#?}", field);
}

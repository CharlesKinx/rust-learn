
use std::cell::Cell;

pub fn print_cell(){
    let c = Cell::new("asdf");
    let one = c.get();
    c.set("qaz");
    let two = c.get();
    println!("{},{}",one,two);
}
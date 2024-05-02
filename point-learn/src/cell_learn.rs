
use std::cell::Cell;
use std::cell::RefCell;
pub fn print_cell(){
    let c = Cell::new("asdf");
    let one = c.get();
    c.set("qaz");
    let two = c.get();
    let three = one;
    println!("{},{},{}",one,two,three);

}

pub fn print_recell() {
    let s = RefCell::new(String::from("hello, world"));
    let s1 = s.borrow();
    let s2 = s.borrow_mut();
    println!("{},{}", s1, s2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_test(){
        print_cell();
    }

    #[test]
    fn two_test(){
        print_recell();
    }
}

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

/**
 *闭包学习
 */

pub struct Cache<T,U>
where T: Fn(U) -> U,
U:Copy
{
    query:T,
    value:Option<U>,
}

impl<T,U> Cache<T,U> 
where
    T: Fn(U) -> U,
    U: Copy
{
    fn new(query:T) -> Cache<T,U> {
        Cache{
            query,
            value:None,
        }
    }
    fn value(&mut self, args:U) -> U{
        match self.value {
            Some(v) => v,
            None =>{
                let v = (self.query)(args);
                self.value = Some(v);
                v
            }
        }
    }
}
pub fn closure_learn() {

    let mut c = Cache::new(|a| a);
    let mut c1 = Cache::new(|a| a);

    let v1 = c.value(2);
    let v2 = c1.value("helloword");

    println!("{v1},{v2}");
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
        //print_recell();

        let s = 23;
        let y = |a| a + s;

        println!("{}",y(7));

        closure_learn()
    }


    #[test]
    fn quote_test() {
        let mut s = String::from("value");
        let s1 = &s;
        println!("{}",s1);

        let s2 = &mut s;
    }
}
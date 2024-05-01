use std::fmt::Debug;
pub fn add<T: std::ops::Add<Output = T>>(a:T,b:T) -> T{
    a + b
}



#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn str_test() {
        let s1 = "test";
        println!("{s1}");
        println!("test1{s1}");
    }
    fn print_it<T: Debug + 'static>( input: &T) {
        println!( "'static value passed in is: {:?}", input );
    }
    #[test]
    fn test1() {
        let i = 6;
        print_it(&6);
    }
}

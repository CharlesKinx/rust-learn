use std::clone::Clone;
use std::borrow::Cow;
pub fn test12() {
    
}
struct Animal<'a> {
    name: Cow<'a, str>,
    age: u32,
    species: Cow<'a, str>,
}


use std::fmt::Debug;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn str_test() {
        let mut animal = Animal {
            name: Cow::Borrowed("Tom"),
            age: 3,
            species: Cow::Borrowed("Cat"),
        };
    
        animal.name.to_mut().push_str("mycat");
        animal.species = Cow::Owned("Lion".to_string());
    
        println!("Name: {}", animal.name);
        println!("Species: {}", animal.species);
    }
    
}
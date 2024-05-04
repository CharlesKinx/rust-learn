struct Interface<'a> {
    manager:&'a mut Manager<'a>
}

impl<'a> Interface<'a> {
    pub fn noop(self) {
        println!("interface consumded");
    }
}

struct Manager<'a> {
    text:&'a str
}
struct List<'a> {
    manager: Manager<'a>
}

impl<'a> List<'a> {
    pub fn get_interface(&'a mut self) ->Interface {
        Interface {
            manager: &mut self.manager
        }
    }
}

fn use_list(list: &List) {
    println!("{}",list.manager.text);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn life_test() {
        let mut list = List{
            manager:Manager{
                text:"Hello"
            }
        };

        list.get_interface().noop();

        println!("1111111111111");
        //use_list(&list);
    }


}
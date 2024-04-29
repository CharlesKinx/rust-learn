use crate::gener::trait_learn::Content;


#[derive(Debug)]
struct User {
    username: String,
    email: String,
    age: u32,
    isalive: bool
}

mod mot;
mod gener;
mod itor;
fn main() {
    // println!("Hello, world!");
    // test();
    // for_test();
    // owner_test();
    // var_scope();

    // let s = String::from("Hello, world!");
    // println!("{s}");
    // owner_parameter(s);
    // println!("{s}");

    // let user1 = User{
    //     username : String::from("whd"),
    //     email : String::from("664410795@qq.com"),
    //     age : 24,
    //     isalive : true
    // };
    // dbg!(&user1);
    // println!("{:#?}",&user1);
    
    let va = gener::add(4,6);
    println!("{va}");
    crate::mot::t2();
    
    let wb = gener::trait_learn::WeiBo{
        title:String::from("微博"),
        content:String::from("微博内容"),
        author:String::from("作者"),
    };
    gener::trait_learn::notify(&wb);
    gener::trait_learn::notify1(&wb);
    let sw = wb.getContent();
    println!("{sw}");


    //vector_test();
}

fn hashmap_test() {
    use std::collections::HashMap;
    //let mut hash = HashMap::new();

}

fn vector_test() {
    let mut v:Vec<i32> = Vec::new();
    for num in 0..5 {
        v.push(num);
    }

    for element in v {
        println!("数字v = {element}");
    }
}

fn owner_parameter(str: String) {
    println!("字符串 = {str}");

    let s = "hello";
    let mut s2 = "hello";
    let s1 = String::from("hello");
}

fn var_scope() {
    let x = 5;
    println!("x = {x}");

    let x = x + 1;
    println!("x = {x}");

    {
        let x = x * 2;
        println!("x = {x}");
    }
    println!("x = {x}");

}

fn owner_test() {
    let x = 4;
    let y = x;
    println!("x = {x}");
    println!("y = {y}");
}

fn for_test() {
    let nums = [1,2,3,4,5,6,7,8,9];
    for num in nums {
        println!("nums = {num}");
    }
}

fn test() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;

        }
        count += 1;
    }
    println!("End count = {count}")
}
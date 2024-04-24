
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

    
    vector_test();
}

fn hashmap_test() {
    use std::collections::HashMap;
    let mut hash = HashMap::new();

}

fn vector_test() {
    let mut v:Vec<i32> = Vec::new();
    for num in (0..5) {
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
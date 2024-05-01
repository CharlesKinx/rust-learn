
#[derive(Debug)]
struct Person{
    name:String,
    age: u8
}

impl Person {
    fn new(name:String,age:u8)->Self{
        Person{name,age}
    }

    fn display(self:&mut Person,age:u8) {
        
    }
}
use std::ops::Deref;

use crate::{cell_learn::print_cell, dr_learn::{Foo, HasDrop1, HasDrop2, HasTwoDrops}};
struct MyBox<T>{
    v:T,
}

impl <T> MyBox<T> {
    fn new(x:T) -> MyBox<T>{
        MyBox{v:x}
    }
}

impl <T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

mod dr_learn;
use std::mem::drop;
mod cell_learn;
fn main() {
    let y = MyBox::new(10);
    println!("{}",*y);

    let _x = HasTwoDrops{
        one:HasDrop1,
        two:HasDrop2,
    };
    let foo = Foo;
    // 主动进行释放，获取foo的所有权
    // 自定义实现的Drop特征中的drop方法只是借用了目标的可变引用
    drop(foo);
    println!("running");

    print_cell();
}

# rust-learn
Rust

### *四月*

|              Mon.              |             Tues.             | Wed. | Thur. | Fri. |                   Sat.                   |             Sun.              |
| :----------------------------: | :---------------------------: | :--: | :---: | :--: | :--------------------------------------: | :---------------------------: |
|               1                |               2               |  3   |   4   |  5   |                    6                     |               7               |
|               8                |               9               |  10  |  11   |  12  |                    13                    |              14               |
|               15               |              16               |  17  |  18   |  19  |                    20                    |              21               |
|               22               |              23               |  24  |  25   |  26  | 27 <br>([D1](#2024427-Day1)) :anguished: | 28<br>([D2](#2024428-Day2)) 🤔 |
| 29<br/>([D3](#2024429-Day3)) 😖 | 30<br>([D4](#2024429-Day4)) 😖 |      |       |      |                                          |                               |



### *五月*

| Mon. | Tues. |            Wed.             |            Thur.             |             Fri.             | Sat. | Sun. |
| :--: | :---: | :-------------------------: | :--------------------------: | :--------------------------: | :--: | :--: |
|      |       | 1<br> ([D5](#2024501-Day5)) | 2<br/> ([D6](#2024502-Day6)) | 3<br/> ([D7](#2024503-Day7)) |  4   |  5   |
|  6   |   7   |              8              |              9               |              10              |  11  |  12  |
|  13  |  14   |             15              |              16              |              17              |  18  |  19  |
|  20  |  21   |             22              |              23              |              24              |  25  |  26  |
|  27  |  28   |             29              |              30              |              31              |      |      |



2024/4/24 从零开始学习Rust，忘记记录了:（。。。希望每天可以记录一下

## 2024/4/27 Day1 

今日rust学习：

- 了解了Rust的生命周期
- panic以及Result

##### rustlings 题目

**进展 72/110 （65.5%）**

```
errors1                 exercises/error_handling/errors1.rs             Done
errors2                 exercises/error_handling/errors2.rs             Done
errors3                 exercises/error_handling/errors3.rs             Done
errors4                 exercises/error_handling/errors4.rs             Done
errors5                 exercises/error_handling/errors5.rs             Done
errors6                 exercises/error_handling/errors6.rs             Done
generics1               exercises/generics/generics1.rs                 Done
generics2               exercises/generics/generics2.rs                 Done
traits1                 exercises/traits/traits1.rs                     Done
traits2                 exercises/traits/traits2.rs                     Done
traits3                 exercises/traits/traits3.rs                     Done
traits4                 exercises/traits/traits4.rs                     Done
traits5                 exercises/traits/traits5.rs                     Done
quiz3                   exercises/quiz3.rs                              Done
lifetimes1              exercises/lifetimes/lifetimes1.rs               Done
lifetimes2              exercises/lifetimes/lifetimes2.rs               Done
lifetimes3              exercises/lifetimes/lifetimes3.rs               Done
tests1                  exercises/tests/tests1.rs                       Done
tests2                  exercises/tests/tests2.rs                       Done
tests3                  exercises/tests/tests3.rs                       Done
tests4                  exercises/tests/tests4.rs                       Done
```

学习感受：rust确实有点难度，感觉很难理解，希望明天能够学习的在多一点

## 2024/4/28 Day2 

今天重新看了一下Rust的所有权以及借用、泛型、特征以及特征对象、生命周期

特征这块还是感觉比较抽象，查阅了部分资料说是与Java的接口类似，抽象出特征出来，一开始按这个思路来学的时候，感觉有些地方能说的通，到特征对象后就感觉越来越抽象了，理解起来还是比较抽象...

浅浅了解了一下生命周期，感觉Rust不需要垃圾回收，主要就是通过生命周期以及所有权...总体来说今天学的东西还是有点晦涩，学了，但又感觉没学...

今天跟着《Rust语言圣经》做了入门的那个demo...感觉还是要知识代入到项目中来学习..

Rustlings今天没有刷，因为刷到迭代器那部分了...明天在开始刷...

## 2024/4/29 Day3

心态炸裂....生命周期...真抽象啊...

今天看了一下迭代器以及闭包，，迭代器还是比较简单的，做题的时候发现了一个很大的问题：

为什么函数返回值不一样，但是代码一样，居然可以通过...

```rust
fn result_with_list() ->  Result<Vec<i32>, DivisionError> {
    let numbers = vec![27, 297, 38502, 81];
    numbers.into_iter().map(|n| divide(n, 27)).collect()
}

// Complete the function and return a value of the correct type so the test
// passes.
// Desired output: [Ok(1), Ok(11), Ok(1426), Ok(3)]
fn list_of_results() ->  Vec<Result<i32,DivisionError>>{
    let numbers = vec![27, 297, 38502, 81];
    numbers.into_iter().map(|n| divide(n, 27)).collect()
}
```

而且，，圣经里面说了collect必须指定类型...怪..

今天学习测试用例的写法了...还行吧..😎

我对Rust的看法：难...语言是个好语言，从他不需要进行垃圾回收，我就觉得这个语言挺厉害的，但是，他的规则以及约定，，好抽象啊...心态炸裂..

今天是学习Rust的第五天...

##### rustlings 题目

**进展 75/110 （68.2%）**

```
iterators1              exercises/iterators/iterators1.rs               Done
iterators2              exercises/iterators/iterators2.rs               Done
iterators3              exercises/iterators/iterators3.rs               Done
iterators4              exercises/iterators/iterators4.rs               Done
```

希望明天能够完成到80题！！



## 2024/4/30 Day4



> 当栈上数据转移所有权时，实际上是把数据拷贝了一份，最终新旧变量各自拥有不同的数据，因此所有权并未转移。
>
> 而堆上则不然，底层数据并不会被拷贝，转移所有权仅仅是复制一份栈中的指针，再将新的指针赋予新的变量，然后让拥有旧指针的变量失效，最终完成了所有权的转移：



今天没怎么学...只是简单看了一下特征对象以及Box只能指针....



##### rustlings 题目

**进展 80/110 （72.7%）**

```
iterators5              exercises/iterators/iterators5.rs               Done
box1                    exercises/smart_pointers/box1.rs                Done
rc1                     exercises/smart_pointers/rc1.rs                 Done
arc1                    exercises/smart_pointers/arc1.rs                Done
cow1                    exercises/smart_pointers/cow1.rs                Done
```

## 2024/5/01 Day5

今天看了Deref、Drop、Rc、Arc，以及关联函数

方法定义在结构体中，如果参数没有self，称该方法为关联函数，使用::来调用，感觉Rust中的关联函数有点像java中的静态方法一样...

Rust编译器会在变量作用域结束的地方调用Drop，进行垃圾回收...这个还是挺厉害的，不过drop方法是借用了目标的可变引用...Rust的Deref还是挺好用的....

```rust
std::mem::drop 可以使用该drop方法获取到所有权，进而进行释放
```



今天的难点在于：特征.... : (..T_T...明天在好好学学特征吧...太难了

明天在看看Cell以及RefCell...



## 2024/5/02 Day6

什么是内部可变性？

String::from和to_string的区别与联系？

> String::from是一个关联函数，用来从一个字符串字面量创建一个`String`。它直接转换一个字符串字面量或者`&str`类型的引用为`String`类型
>
> 这是实现了`ToString` trait的所有类型的一个方法，它允许我们把任何实现了`Display` trait的类型转换为`String`。这意味着任何你可以打印出来的类型（即实现了`Display`），比如整型、浮点型、结构体等，都可以使用`to_string`来转换为`String`



&s与s.borrow();的区别与联系？

1. `&s`: 直接取得了`s`的不可变引用（借用），这是Rust核心语法的一部分。你可以使用`&s`来借用任何类型的值，不仅限于实现了`Borrow` trait的类型。
2. `s.borrow()`: 这是通过`Borrow` trait提供的一个方法。当你调用`s.borrow()`时，你实际上是调用了该类型实现的`Borrow` trait的`borrow`方法。`Borrow` trait通常用于泛型编程，允许函数接受既可以是直接类型也可以是引用类型的参数。



`Cell` 只适用于 `Copy` 类型，用于提供值，而 `RefCell` 用于提供引用

`Cell` 不会 `panic`，而 `RefCell` 会

## 2024/5/03 Day7

闭包是**一种匿名函数，它可以赋值给变量也可以作为参数传递给其它函数，不同于函数的是，它允许捕获调用者作用域中的值**，闭包的

- move关键字，move关键字会强制闭包获取所用到的环境变量的所有权
- Fn，闭包捕获变量有三种途径，恰好对应函数参数的三种传入方式：转移所有权、可变借用、不可变借用，因此相应的 `Fn` 特征也有三种：

**闭包的三种特征...看了还是感觉有点抽象...明天再看**，下次一定

`Weak` 非常类似于 `Rc`，但是与 `Rc` 持有所有权不同，`Weak` 不持有所有权，它仅仅保存一份指向数据的弱引用：如果你想要访问数据，需要通过 `Weak` 指针的 `upgrade` 方法实现，该方法返回一个类型为 `Option<Rc<T>>` 的值

### 阶段性总结：

#### 问题：

学的有点慢，吃力，概念记不住...

学完感觉脑子一片空白...

遇见抽象的就看不下去了，静不下心来去学

每天也不知道该记录什么....

#### 下一阶段目标：

根据目标来进行学习，给自己一个ddl【比如今天要看完哪些...】

每天对自己学完的东西进行一个回看...记录让自己不懂、吃力、难理解的东西【可以不解决，但一定要记录】**【强制做】**

#### 需要做的事情：

- [x] 再次尝试生命周期
- [x] 闭包3个Fn的特征
- [ ] 迭代器

- [ ] 模式匹配
- [ ] 再看特征

- [ ] 并发与并行
- [ ] 使用多线程
- [ ] 线程同步：消息传递

- [ ] 线程同步：锁、Condvar、信号量
- [ ] 线程同步：原子操作
- [ ] 基于send和Sync的线程安全
- [ ] 全局变量
- [ ] 错误处理
- [ ] Unsafe
- [ ] Macro 宏编程
- [ ] 异步编程

## 2024/5/04 Day8

rust在1.31版本引入了NLL机制，该机制可以让引用的生命周期停止在最后一次调用的地方，而不是该引用的作用域中。rust还提供了生命周期消除，`'a:'b` 说明a的生命周期要比b的生命周期要长，拥有a的变量生命周期可以向下转化为b。

Rust生命周期的一点理解：

对于拥有所有权的变量来说，生命周期只有离开作用域或者被赋予其他变量了，该变量就没有生命周期了，也就没法使用了

对于引用来说，引用的生命周期比较复杂，一方面是通过拥有所有权来获取，该生命周期的会持续到最后一次使用，或作用域结束，如果通过函数参数来获取，那么该变量的生命周期与参数相同。如果结构体中有引用，该引用 的生命周期要小于结构体的生命周期，可以通过生命周期消除来改变。



#### 闭包Fn的三种特征

1、FnOnce  该类型的闭包会拿走被捕获变量的所有权。`Once` 顾名思义，说明该闭包只能**运行一次**，**仅**实现 `FnOnce` 特征的闭包在调用时会转移所有权，所以显然不能对已失去所有权的闭包变量进行二次调用：

可以使用move关键字，强行拿到捕获变量的所有权，在闭包外部，该变量就失效了

2、`FnMut`，它以可变借用的方式捕获了环境中的值，因此可以修改该值

3、`Fn` 特征，它以不可变借用的方式捕获环境中的值 

##### 三者的关系：

FnOnce > FnMut > Fn，Fn是其他两个的子集

**一个闭包实现了哪种Fn，取决于该闭包如何处理被捕获的变量**，如果该闭包获取了该变量的所有权，则处于第一种，如果该闭包获取了变量的可变引用，属于第二种。否则是第三种。

- 所有闭包都自动实现了FnOnce的特征，任何一个闭包都至少可以被调用一次
- 没有获得被捕获变量的所有权，自动实现了FnMut特征
- 不需要对捕获变量进行改变的闭包自动实现了Fn特征



##### 额外收获：

```go
fn do1(c: String) {}：表示实参会将所有权传递给 c
fn do2(c: &String) {}：表示实参的不可变引用（指针）传递给 c，实参需带 & 声明
fn do3(c: &mut String) {}：表示实参可变引用（指针）传递给 c，实参需带 let mut 声明，且传入需带 &mut
fn do4(mut c: String) {}：表示实参会将所有权传递给 c，且在函数体内 c 是可读可写的，实参无需 mut 声明
fn do5(mut c: &mut String) {}：表示实参可变引用指向的值传递给 c，且 c 在函数体内部是可读可写的，实参需带 let mut 声明，且传入需带 &mut
```


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

| Mon. | Tues. |            Wed.             |            Thur.             | Fri. | Sat. | Sun. |
| :--: | :---: | :-------------------------: | :--------------------------: | :--: | :--: | :--: |
|      |       | 1<br> ([D5](#2024501-Day5)) | 2<br/> ([D6](#2024502-Day6)) |  3   |  4   |  5   |
|  6   |   7   |              8              |              9               |  10  |  11  |  12  |
|  13  |  14   |             15              |              16              |  17  |  18  |  19  |
|  20  |  21   |             22              |              23              |  24  |  25  |  26  |
|  27  |  28   |             29              |              30              |  31  |      |      |



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

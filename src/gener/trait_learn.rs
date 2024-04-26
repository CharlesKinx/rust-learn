
pub trait Content {
    fn getContent(&self) ->String;
}

pub trait TimePub {
    fn pubContent(&self) ->String;
}

pub struct WeiBo {
    pub title:String,
    pub content:String,
    pub author:String
}

pub struct Post{
    title:String,
    content:String,
    author:String
}

impl TimePub for WeiBo {
    fn pubContent(&self) ->String {
        format!("在规定时间内发布，文章标题为{},作者是{},内容是{}",self.title,self.author,self.content)
    }
}

impl Content for WeiBo {

    fn getContent(&self) -> String {
        format!("文章标题{},作者{},内容{}",self.title,self.author,self.content)
    }
}

impl Content for Post {

    fn getContent(&self) -> String {
        format!("作者{},文章标题{},内容{}",self.author,self.title,self.content)
    }
}

/**
 * 使用特征作为函数参数
 */
pub fn notify(item :&impl Content) {
    println!("{}",item.getContent());
}

/**
 * 特征约束
 */
pub fn notify1<T: Content>(item: &T) {
    println!("Breaking news! {}", item.getContent());
}

pub fn notify2<T: Content>(item1: &T, item2: &T) {

}

/**
 * 多重约束约束
 * 必须同时实现这两个trait才可以调用该方法
 */
pub fn notify3(item: &(impl Content + TimePub)) {}

/**
 * 函数返回中的 impl Trait
 */
pub fn get_wei_bo() -> impl Content {
    WeiBo {
        title:String::from("qas"),
        content:String::from("qw"),
        author:String::from("ws"),
    }
}
fn fn_once<F>(func: F)
where
    F: FnOnce(String),
{
    func("hello".to_string());
}

fn test_fn() {
    let s = String::from("test11111");

    let update_string =  || println!("{}",s);

    exec(update_string);
    println!("{}",s);
    exec1(update_string);
    exec2(update_string);
}

fn exec<F: FnOnce()>(f: F)  {
    f()
}

fn exec1<F: FnMut()>(mut f: F)  {
    f()
}

fn exec2<F: Fn()>(f: F)  {
    f()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        test_fn();
    }

    #[test]
    fn test2() {
        let  s = String::from("test11111");
        let mut fn_t = || {
            println!("{s}");
        };
        
        println!("{s}");
    }
}

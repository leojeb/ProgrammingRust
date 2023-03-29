use std::rc::Rc;

fn main () {
    t1();
}


fn t1 () {

    let s: Rc<String>  = Rc::new("shirataki".to_string());// s是指针
    let t: Rc<String> = s.clone();
    let u: Rc<String> = s.clone();
    // s, t, u都指向同一块堆内存
    // Rc创建出来的类型可以直接使用原来类型的方法
    assert!(s.contains("shira"));
    assert_eq!(t.find("taki"), Some(5));
    println!("{}", u); 
    // 但是 Rc<T> 持有的指针是不可变的，所以我们不能更改所包含的字符串：

    s.push_str(" noodles");

}

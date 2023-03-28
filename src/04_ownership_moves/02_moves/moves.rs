fn main () {
    more_examples();
}
 
fn move_objects () {
    let s = vec![
        "a".to_string(), 
        "b".to_string(), 
        "c".to_string(), 
    ];
    let t = s;
    // let u = s; s的值被移动到t，现在s未初始化，赋值给u会报错。

}


fn more_examples () {
    fn a (x: Vec<T>) {
        println!("{}", x)
    }
    
    let x = vec![1,2,3];
    if 1 {
        a(x)
    } else {
        b = a;
        b(x)
    }
    
    h(x);// x已经被a或b使用，这里会报错
    

    while 1 {
        g(x);
        x = h();
    }
    e(x);


}

fn t1 () {
    // in rust, 引用是显示声明, 显示解引用的
    let x = 10;
    let r = &x;
    assert!(*r==10);
    // 创建一个可变引用
    let mut y = 20;
    let m = &mut y;
    *m += 32;
    assert!(*m==52);
    assert!(y==52);
}

fn t2 () {
    /*
        引用永远不为null, rust没有表示null pointer的值, rust使用Option<&T> 来表示一个可能是引用或者不引用任何内容的值,
        看到Option<&T>就意味着需要检验是否为null指针
     */
}

fn factorial(n: usize) -> usize {
    (1..n+1).product()
}

fn t3 () {
    /*
    rust的指针可以指向任意的表达式
        In situations like this, Rust simply creates an anonymous
    variable to hold the expression’s value and makes the
    reference point to that. The lifetime of this anonymous
    variable depends on what you do with the reference:
    If you immediately assign the reference to a variable
    in a let statement (or make it part of some struct or
    array that is being immediately assigned), then Rust
    makes the anonymous variable live as long as the
    variable the let initializes. In the preceding
    example, Rust would do this for the referent of r.
    Otherwise, the anonymous variable lives to the end
    of the enclosing statement. In our example, the
    anonymous variable created to hold 1009 lasts only
    to the end of the assert_eq! statement.
     */
    let r  = &factorial(6);
    // 算术运算符可以看穿一层引用,直接使用引用内容的值
    assert_eq!(r + &1009, 1729) 
}

fn t4 () {
    /*
        rust还包含一种'胖指针', 简单的引用只是存储一个地址,而胖指针存储了额外的信息
        1. 引用slice的指针, 会携带slice的地址信息和它的length信息.
        2. trait object(特性对象), 引用一个实现了某种特性的值的指针.后面Trait Object会讲到
        除此之外,胖指针和简单指针是一样的
     */
}



fn main () {
    t1();
}




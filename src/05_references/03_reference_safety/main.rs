fn t1() {
    {
        let r;
        {
            let x = 1;
            r = &x;
            assert_eq!(*r, 1); // 这里不报错
        }   
        // assert_eq!(*r, 1); // bad: reads memory `x` used to occupy, x已经无了. 
    }
}

// 全局变量
static mut STASH: &i32 = &128;
fn t2(p: &'static i32) {
    unsafe {
        // p的生命周期应该和全局变量STASH相同, 都是static,这样才不会有空指针
        STASH = p;
    }
    
}

fn t3() {
    static WORTH_POINTING_AT: i32 = 100;
    t2(&WORTH_POINTING_AT); // 必须传入一个具有static生命周期的值的引用
}

fn t4(v: &[i32]) -> &i32 {
    let mut s = &v[1];
    for r in &v[1..] {
        if *r < *s { s = r;}
    }
    s
}

// fn t5() {
//     let s;
//     {
//         let a: Vec<i32> = vec!{1, 2, 3};
//         s = t4(&a);
//         assert_eq!(*s, 1); // 这里不报错, 并不要求s和a的lifetime完全一致, 只需要a比s活得长就行了
//     }
//     assert_eq!(*s, 1); // 这里报错
// }

// struct containing references
/*
    a good practice about mut and shared references
 */
fn t6() {
    let b1 = "asf".to_string();
    println!("b1 {:}", *b1);
    let mut a = "as".to_string();
    println!("a {:}", a);
    let a1 = a;
    println!("a1 {:}", a1);
    a = "afasf".to_string();
    println!("a {:}", a); 
    println!("a1 {:}", a1);
    let mut b = "5";
    b = "6";
    let c = "3";
    // c = "5"; error
    let mut v1 = vec![
        "1".to_string(),
        "2".to_string(),
        "3".to_string()
    ];
    for i in &mut v1 {
        *i = "4".to_string()
    }
    println!("{:?}", v1);


    let mut v2 = vec!["1","2","3"];
    for mut i in &mut v2 {
        i = &mut "5";
    }
    println!("{:?}", v2);

    let mut v3 = vec!["1","2","3"];
    for i in &mut v3 {
        *i = &mut "5";
    }
    println!("{:?}", v3);

}

fn main() {
    // t4(&vec![1,2]);
    t6()
}


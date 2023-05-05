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
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s { s = r;}
    }
    s
}

fn t5() {
    let s;
    {
        let a: Vec<i32> = vec!{1, 2, 3};
        s = t4(&a);
        assert_eq!(*s, 1); // 这里不报错, 并不要求s和a的lifetime完全一致, 只需要a比s活得长就行了
    }
    assert_eq!(*s, 1); // 这里报错
}

// struct containing references
fn t6() {
     
}

fn main() {
    t5();
}


use std::collections::HashMap;
fn main()  {
    #[derive(Debug)]
    struct A {
        a: String,
        b: i64,
        c: String
    }
    #[derive(Debug)]
    struct B {
        b: HashMap<String, A>
    }
    // let a1:A = Default::default();
    let b1:B =  B {
        b: Default::default()
    };
    // println!("a{:?}", a1);
    println!("b{:?}", b1.b);
}



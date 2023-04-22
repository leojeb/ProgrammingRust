use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn show(table: Table) {
    for (artist, works) in table {
        println!("{}:", artist);
        for work in works {
            println!("{}", work)
        }
    }
}

fn main () {
    let mut t = Table::new();
    t.insert(
        "a".to_string(), vec!["1".to_string(), "2".to_string(), "3".to_string()]
    );
    show(t);
    // show(t);

    t2();
    t3();
    t5();
}


struct Anime {
    a: &'static str,
    b: &'static str
}    

fn t2 () {
    let anime = Anime { a: "123", b:"234"};
    let anime_ref = &anime;
    assert_eq!(anime.a, (*anime_ref).a)
}

fn t3 () {
    let x = 1;
    let y = 2;
    let mut c = &x;
    if true {
        c = &y;
    }
    assert_eq!(*c,2);
}

fn t4 () {
    struct Point { x: i32, y: i32 }
    let point = Point { x: 1000, y: 729 };
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr;
}

fn t5 () {
    let a = 1;
    let b = 1;
    let ra = &a;
    let rb = &b;
    // 判断引用是否相等，会直接将引用的值比较
    assert_eq!(ra, rb);
    // 如果想判断引用地址，则如下
    // assert!(std::ptr::eq(ra, rb));
    assert!(ra == &ra); // error: type mismatch: `&i32` vs `&&i32`
}



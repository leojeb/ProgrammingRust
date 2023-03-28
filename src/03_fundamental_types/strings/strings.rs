
fn main () {
    // string literal
    let speech = "\"Ouch!\", said 
        the well.\n";
    println!("{}",speech);
    println!("asfd \
        asf");

    // raw string
    let rs = r"abn\n";
    println!("{}", rs);
    // let pattern = Regex::new(r"\d+(\.\d+)*");


    // byte strings
    let method = b"GET";
    assert_eq!(method, &[b'G',b'E', b'T']);
    let bs = br"a\n1";
    assert_eq!(bs, &[b'a',b'\\',b'n',b'1']);
    //  &String to &str automatically

    // strings in memory
    assert_eq!("青色".len(), 6);
    assert_eq!("青色".chars().count(), 2);

    let mut s = "hello";
    let mut s1 = &mut("hello".to_string()); // 可写指针
    // let mut s1 = &("hello".to_string()); &是只读指针
    // s[0] = 'c';
    // s1[0] = 'c';
    // s.push('1');
    s1.push('2');

    // String as a Vec<u8> that is guaranteed to hold well-formed UTF-8; in fact, this is
    // how String is implemented, &String 会变成 &str 类型， 就像Vec<T> 调用函数时会自动变成
    // to_string converts &str to String
    let error_msg = "too many pets".to_string();
    let error_msg = "too many pets".to_owned();
    // format! returns a String
    assert_eq!(format!("{}{}{}", 1, 2, 3), "123".to_string());

    // Arrays, slices, and vectors of strings have two methods, .concat()
    // and .join(sep), that form a new String from many strings
    let new_s = ["123", "234"].join("");
    println!("{}", new_s);

    // Strings support the == and != operators, Strings also support the comparison operators <, <=, >, and >=
    assert!("ONE".to_lowercase() == "one");

    assert!("peanut".contains("nut"));
    assert_eq!("■_■".replace("■" ," ޵"), " ޵_ ޵");
    assert_eq!(" clean\n".trim(), "clean");
    for word in "veni, vidi, vici".split(", ") {
        assert!(word.starts_with("v"));
    }

}


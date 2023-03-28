fn t1 () {
    // char to int
    assert_eq!('*' as i32, 42);
    // assert_eq!('޵' as u16, 0xca0);
    assert_eq!('錆' as i8, 0x6); // U+0CA0 truncated to eight bits, signed
    // only u8 can use as to convert to char
    assert_eq!(32_u8 as char, ' ');
    assert_eq!('*'.is_alphabetic(), false);
    assert_eq!('β'.is_alphabetic(), true);
    assert_eq!('8'.to_digit(10), Some(8));
    // assert_eq!('޵ '.len_utf8(), 3);
    assert_eq!(std::char::from_digit(2, 10), Some('2'));
}

fn main () {
    t1()
}


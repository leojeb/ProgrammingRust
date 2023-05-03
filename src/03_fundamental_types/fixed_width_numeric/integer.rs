/*
   u8           (0 to 255) byte
   u16         (0 to 65,535)
   u32          (0 to 4,294,967,295)
   u64         (0 to 18,446,744,073,709,551,615, or 18
               quintillion)
   u128         (0 to around 3.4✕10 )
   usize        (0 to 4,294,967,295 or 0 to 18,446,744,073,709,551,615)
*/

/*
   i8           (−128 to 127)
   i16          (−32,768 to 32,767)
   i32          (−2,147,483,648 to 2,147,483,647)
   i64          (−9,223,372,036,854,775,808 to
       9,223,372,036,854,775,807)
   i128         (roughly -1.7✕10 to +1.7✕10 )
   isize        Either (−2,147,483,648 to 2,147,483,647), or (−9,223,372,036,854,775,808 to
       9,223,372,036,854,775,807)
*/
fn test1() {
    let a = 42u8;
    let b = 12985isize;
    // 如果不指明则默认类型为i32.
    let c = 0x12_344_35_3651u64; // 0x prefix 16进制,u32 suffix
                                 // byte
    let ascii = b'\x1b'; // = 27, 为了表明是ASCII码所以这样写
    let asc1 = b'A';
    let asc2 = 65u8;
    let asc3 = b'\x41';
    assert_eq!(asc1, asc2);
    assert_eq!(asc3, asc2);
    // assert_eq!(ascii, asc2);
}

fn test2 () {
    assert_eq!( 1000_i16 as u8, 232_u8); 
    // truncation happens if out of range
    assert_eq!(65535_u32 as i16, -1_i16);
    assert_eq!( -1_i8 as u8, 255_u8);
    assert_eq!( 255_u8 as i8, -1_i8);
    assert_eq!( 2_u16.pow(4), 16);
    assert_eq!( (-4_i32).abs(), 4);
    assert_eq!( 0b_010101_u8.count_ones() as i8, 3);

    // When an integer arithmetic operation overflows, Rust panics, in a debug build. In a
    // release build, the operation wraps around: it produces the value equivalent to the
    // mathematically correct result modulo the range of the value. (In neither case is over‐
    // flow undefined behavior, as it is in C and C++.)
    // For example, the following code panics in a debug build:
    let mut i = 1;
    loop {
        i *= 10; // panic: attempt to multiply with overflow
        // (but only in debug builds!)
    }
    // In a release build, this multiplication wraps to a negative number, and the loop runs
    // indefinitely
    
}

fn t3() {
    // Checked Operations

    assert_eq!(10_u8.checked_add(20), Some(30));
    assert_eq!(100_u8.checked_add(190), None);
    // let sum = 60_u8.checked_add(500).unwrap(); panic if ofr
    assert_eq!(10_i16.checked_div(-1), Some(-10));


    // Wrapping Operations

    assert_eq!(100_u16.wrapping_mul(200), 20000);
    assert_eq!(500_u16.wrapping_mul(500), 53392);
    assert_eq!(500_u16.wrapping_mul(500), 53392);

    // The first product can be represented as a u16;
    // the second cannot, so we get 250000 modulo 2¹⁶.
    assert_eq!(100_u16.wrapping_mul(200), 20000);
    assert_eq!(500_u16.wrapping_mul(500), 53392);
    // Operations on signed types may wrap to negative values.
    assert_eq!(500_i16.wrapping_mul(500), -12144);
    // In bitwise shift operations, the shift distance
    // is wrapped to fall within the size of the value.
    // So a shift of 17 bits in a 16-bit type is a shift
    // of 1.
    assert_eq!(5_i16.wrapping_shl(17), 10);

    // Saturating Operations

    // Saturating operations return the representable value that is closest to the mathe‐
    // matically correct result. In other words, the result is “clamped” to the maximum
    // and minimum values the type can represent:
    assert_eq!(32760_i16.saturating_add(10), 32767);
    assert_eq!((-32760_i16).saturating_sub(10), -32768);
    // There are no saturating division, remainder, or bitwise shift method


    // Overflowing operations return a tuple (result, overflowed), where result is
    // what the wrapping version of the function would return, and overflowed is a
    // bool indicating whether an overflow occurred:
    assert_eq!(255_u8.overflowing_sub(2), (253, false));
    assert_eq!(255_u8.overflowing_add(2), (1, true));
    // overflowing_shl and overflowing_shr deviate from the pattern a bit: they
    // return true for overflowed only if the shift distance was as large or larger than
    // the bit width of the type itself. The actual shift applied is the requested shift mod‐
    // ulo the bit width of the type:
    // A shift of 17 bits is too large for `u16`, and 17 modulo 16 is 1.
    assert_eq!(5_u16.overflowing_shl(17), (10, true));

}


fn main() {
    // #[test]
    
    test1();
    // test2();
    let x = (-4_i32).abs();
    println!("{}",x);
    t3();
    /*
        Addition             add         100_i8.checked_add(27) == Some(127)
        Subtraction          sub         10_u8.checked_sub(11) == None
        Multiplication           mul         128_u8.saturating_mul(3) == 255
        Division             div         64_u16.wrapping_div(8) == 8
        Remainder            rem         (-32768_i16).wrapping_rem(-1) == 0
        Negation             neg         (-128_i8).checked_neg() == None
        Absolute             value       abs (-32768_i16).wrapping_abs() == -32768
        Exponentiation           pow         3_u8.checked_pow(4) == Some(81)
        Bitwise             left        shift shl 10_u32.wrapping_shl(34) == 40
        Bitwise             right       shift shr 40_u64.wrapping_shr(66) == 10
     */
    
}


/*
    u8           0 to 2 –1 (0 to 255) byte
    u16          0 to 2 −1 (0 to 65,535)
    u32          0 to 2 −1 (0 to 4,294,967,295)
    u64          0 to 2 −1 (0 to 18,446,744,073,709,551,615, or 18
                quintillion)
    u128         0 to 2 −1 (0 to around 3.4✕10 )
    usize        0 to either 2 −1 or 2 −1
 */

/*
    i8           −2 to 2 −1 (−128 to 127)
    i16          −2 to 2 −1 (−32,768 to 32,767)
    i32          −2 to 2 −1 (−2,147,483,648 to 2,147,483,647)
    i64          −2 to 2 −1 (−9,223,372,036,854,775,808 to
        9,223,372,036,854,775,807)
    i128         −2 to 2 −1 (roughly -1.7✕10 to +1.7✕10 )
    isize        Either −2 to 2 −1, or −2 to 2 −1
 */
#[test]
fn test1() {
    let mut a = 42u8;
    let mut b = 12985isize;
    // 如果不指明则默认类型为i32.
    let mut c = 0x12_344_35_3651u32; // 0x prefix 16进制,u32 suffix
    // byte
    let mut ascii = b'\x1b'; // = 27, 为了表明是ASCII码所以这样写
    let asc1 = b'A';
    let asc2 = 65u8;
    let asc3 = b'\x41';
    assert_eq!(asc1, asc2, asc3)
}

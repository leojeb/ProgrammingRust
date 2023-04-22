fn main() {
    // 定义
    let a1: [u32; 6] = [1, 2, 3, 4, 5, 6];
    let tax = ["Animal", "test1", "create"];
    assert_eq!(a1[1], 2);
    assert_eq!(tax.len(), 3);
    
    let mut l1 = [true; 10000];
    for i in 1..100 {
        if l1[i] {
            let mut j = i * i;
            while j < 10000 {
                l1[j] = false;
                j += i;
            }
        }
    }
    // assert!(l1[234]);
    assert!(!l1[9876]);
    // println!("{}",l1)
    // rust没有空array， 而且长度是固定的，可以定义成[0u8; 1024]
    let mut chaos = [1,2,5,3,12,1];
    // Rust implicitly produces a &mut [i32] slice referring to the entire array
    // and passes that to sort to operate on
    chaos.sort();
    assert_eq!(chaos, [1,1,2,3,5,12]);
}

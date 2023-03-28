fn main() {
    // 1. 创建vector
    let mut v: Vec<u32> = Vec::new();
    let mut v1 = vec![1, 2, 3];
    let mut v2: Vec<i32> = (0..5).collect();
    // 2. vector里包含头指针，容量capacity和实际元素个数（也即是len），超过容量会导致reallocation，
    // 分配新的内存空间并将原vec里的元素复制过去，然后再释放旧的vec， 所以如果事先知道元素个数，可使用如下来创建vec
    let mut v3: Vec<u16> = Vec::with_capacity(10);
    assert_eq!(v1.iter().product::<i32>(), 6);
    // Here, the reverse method is actually defined on slices, but the call implicitly borrows
    // a &mut [&str] slice from the vector and invokes reverse on that
    // A palindrome!
    let mut palindrome = vec!["a man", "a plan", "a canal", "panama"];
    palindrome.reverse();
    // Reasonable yet disappointing:
    assert_eq!(palindrome, vec!["panama", "a canal", "a plan", "a man"]);
    // Here, the reverse method is actually defined on slices, but the call implicitly
    // borrows a &mut [&str] slice from the vector and invokes reverse on that
    // &Vec<T> to &[T] automatically
    // 增删改查
    v1.push(11);

    v = Vec::with_capacity(2);
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 2);

    v.push(1);
    assert_eq!(v.len(), 1);
    assert_eq!(v.capacity(), 2);
    v.push(2);
    v.push(3);
    assert_eq!(v.len(), 3);
    assert_eq!(v.capacity(), 4); // the capacity isn't guaranteed to be 4
    assert_eq!(v, [1, 2, 3]);

    v.insert(1, 35);
    v.remove(2);
    let pop_value = v.pop();
    assert_eq!(v, [1, 35]);
    assert_eq!(pop_value, Some(3));

    // iterate
    let lang: Vec<String> = std::env::args().skip(1).collect();
    for l in lang {
        println!(
            "{}:{}",
            l,
            if l.len() % 2 == 0 {
                "functional"
            } else {
                "imperative"
            }
        )
    }
}

fn new_pixel_buffer(rows: usize, cols: usize) -> Vec<u8> {
    vec![0; rows * cols]
}

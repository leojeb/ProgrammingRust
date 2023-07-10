
// fn main() {
//     println!("Hello, world!");
// }

// fn gcd(mut n: u64, mut m: u64) -> u64 {
//     assert!(n != 0 && m != 0);
//     while m != 0 {
//         if m < n {
//             let t: u64 = m;
//             m = n;
//             n = t;
//         }
//         m = m % n;
//     }
//     {
//         println!("求得最大公约数为:{n}");
//         n
//     }
// }

// #[test]
// fn test_gcd() {
//     assert_eq!(gcd(14,15) ,1)
// }

use std::any::type_name;

// 定义一个结构体
struct User {
    id: i32,
    name: String,
}

fn t()  {
        
        // 构造一个实例
    let user = User {
        id: 1,
        name: "Alice".into(),
    };

    // 获取结构体类型名称
    let struct_type = type_name::<User>();
    println!("Struct Type: {}", struct_type);

    // 判断结构体是否包含某个属性
    let has_id = struct_type.contains("id");
    let has_age = struct_type.contains("age");
    println!("Has id: {}", has_id);
    println!("Has age: {}", has_age);
        
}



fn main()  {
    t();
}

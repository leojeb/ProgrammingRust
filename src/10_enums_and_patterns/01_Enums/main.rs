use std::collections::HashMap;
enum Pet {
    Dog,
    Cat
}
use self::Pet::*;

fn main()  {
    // use repr to cover default type(i8) of these enum value
    #[repr(i16)]
    enum Ordering {
        Less,
        Greater,
        Equal
    }

    println!("OK {:}", HttpStatus::Ok as i8);
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum TimeUnits {
    Seconds, Mins, Hours
}
fn t1 () {
    let four_years_ago = RoughTime::Past(TimeUnits::Hours, 4 * 365 * 24);
}

pub enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    NotModified = 304
}
pub fn http_status_from_u32(n: u32) -> Option<HttpStatus> {
    match n {
        200 => Some(HttpStatus::Ok),
        _ => None
    }
}

/*
    枚举值：
        1. 类似tuple struct
        2. 类似普通struct, 有属性名和类型
        3. 类似unit struct

*/
// 枚举数据，枚举值可以带参数
enum RoughTime {
    Past(TimeUnits, i32),
    Now,
    Future(TimeUnits, u32)
}
struct Point(u32, u32);
enum Shape {
    Sphere { radius: u64, center: Point},
    Rectangle { width: u64, height: u64}
}

enum RelationshipStatus{
    Single,
    InRelation,
    Broken,
    ItsComplicated(Option<String>),
    ExetremelyComplicated {
        lover: String,
        lover1: String
    }
}

// enum RelationshipStatus {
//     Single,
//     InARelationship,
//     ItsComplicated(Option<String>),
//     ItsExtremelyComplicated {
//         car: DifferentialEquation,
//         cdr: EarlyModernistPoem,
//     },
// }

/*
    泛型枚举
*/
enum Result<T, E> {
    Ok(T),
    Err(E)
}
enum Option<T> {
    Some(T),
    None
}
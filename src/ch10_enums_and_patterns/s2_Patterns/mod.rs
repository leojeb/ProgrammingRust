use std::cmp::Ordering::*;


/**
 * 字面量通配符匹配
 */

#[test]
fn t1() {
    let a = "asf";
    match a {
        "asf" => {println!("1{:}", 1);},
        "adg" => {},
        other => {println!("{:}", other);}
    }
}

/**
 * tuple, struct 匹配
 */
pub fn describe_points(x: i32, y: i32) {
    match (x.cmp(&0), y.cmp(&0)) {
        (Equal, Equal) => "at the origin",
        (_, Equal) => "on the x axis",
        (Equal, _) => "on the y axis",
        other => ""
    };
}
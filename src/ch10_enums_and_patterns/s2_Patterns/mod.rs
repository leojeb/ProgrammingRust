
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
#[test]
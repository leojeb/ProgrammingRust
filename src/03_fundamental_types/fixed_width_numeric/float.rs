// -1.5625              Inferred −(1 9⁄16)
// 2.                   Inferred 2
// 0.25                 Inferred ¼
// 1e4                  Inferred 10,000
// 40f32                f32 40
// 9.109_383_56e-31f64  f64 Roughly 9.10938356 × 10–31

fn t1 () {
    assert!((-1. / f32::INFINITY).is_sign_negative());
    assert_eq!(f32::Min, f32::Max);
    
    assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.);
    assert_eq!((-1.01f64).floor(), -2.);
    // 声明类型
    println!("{}", (2.0_f64).sqrt());
    println!("{}", f64::sqrt(2.0));
    // 必须显示转换
    println!(5_i16 as i32);

}

fn main() {
    t1();
}
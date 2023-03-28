fn main() {
    let v: Vec<f64> = vec![1., 2., 3., 0.707];
    let a: [f64; 4] = [1 as f64, 2.0, 3.0, 4.0];
    let sv: &[f64] = &v;
    let sa: &[f64] = &a;
    print(&v);
    print(&v[0..2]);
}

use std::error::Error;
use std::fs::File;
use std::io::Write;
fn main() {}

type RResult<T> = Result<T, dyn Error>;
/**
 * trait
 */
// trait Write {
//     fn write(&mut self, buf: &[u8]) -> RResult<usize>;
//     fn flush() -> () {}
//     fn write_all() -> () {}
// }
pub fn t1(out: &mut dyn Write) {
    out.flush().unwrap();
}

pub fn t2() {
    let mut buf: Vec<u8> = vec![];
    buf.write_all(b"asdfjoi").unwrap();

    // let writer: dyn Writer = buf; unknown size of buf
    let writer1: &mut dyn Write = &mut buf; // ref is sized
}

/**
 * 泛型函数
 */
pub fn t3() {
    let mut local_file = File::create("a.txt").unwrap();
    // 普通函数
    fn say_hello(writer: &mut dyn Write) {}
    say_hello(&mut local_file);
    //泛型函数
    fn say_hellp<W: Write>(writer: &mut W) {}
    say_hellp(&mut local_file);

    // let v1 = (0..100).collect(); collect函数无法进行类型推断，只有要执行了才知道是什么类型
    let v1: Vec<u8> = (0..100).collect(); //ok
    let v2 = (0..100).collect::<Vec<u8>>(); //ok
}

/**
 * 多继承
 */
pub fn t4() {
    use std::fmt::Debug;
    use std::hash::Hash;
    fn top_ten<T: Debug + Hash + Eq>(values: &Vec<T>) {}
}

/// Run a query on a large, partitioned data set.
/// See <http://research.google.com/archive/mapreduce.html>.
/// rust 有做大数据的潜力呀
fn run_query<M: Mapper + Serialize, R: Reducer + Serialize>(
    data: &DataSet,
    map: M,
    reduce: R,
) -> Results {
}

fn run_query<M, R>(data: &DataSet, map: M, reduce: R) -> Results
where
    M: Mapper + Serialize,
    R: Reducer + Serialize,
{
    // do something
}

/// Return a reference to the point in `candidates` that's
/// closest to the `target` point.
fn nearest<'t, 'c, P>(target: &'t P, candidates: &'c [P]) -> &'c P
where
    P: MeasureDistance,
{
    // ...
}

// 结构体不是泛型，方法也可以是泛型的
struct A {

}
impl A {
    pub fn test<T: Topping>(&mut self, topping: T) {
        // do sth
    }
}

//

#[test]
fn t1()  {
    tuple_struct();
    unit_struct();
}

/// 和java不一样, pixels(引用值)和size的值是内嵌在结构体中的,体现rust独有的所有权机制
#[derive(Debug, Clone)]
struct GrayScaleMap {
    pixels: Vec<u8>,
    size: (usize, usize)
}

fn new_map(size:(usize, usize), pixels: Vec<u8>) -> GrayScaleMap{
    assert_eq!(pixels.len(), size.0 * size.1);
    GrayScaleMap{pixels, size }// 局部变量名和名字相同，省略
}

fn normal_struct()  {
    let width = 1024;
    let height = 576;
    let image = GrayScaleMap{
        pixels: vec![0;width * height],
        size: (width, height)
    };
    fn new_grayscalemap(size: (usize, usize), pixels: Vec<u8>) -> GrayScaleMap {
        return GrayScaleMap {
            pixels, size
        };
    }
    // 直接..a将同类型变量a的值赋给i2,所有未显示赋值的字段都取a的值
    // 注意！a中那部分赋值给i2的内容是直接move给i2的，所以现在a是partially moved
    // 所以最好加clone
    let i2 = GrayScaleMap {
        size: (1,2),
        ..image.clone()
    };
    println!("image:{:?}, i2:{:?}", image, i2);

}


fn tuple_struct()  {
    struct SingleTuple(pub u32);
    struct Bound(pub usize, pub usize);
    let b1 = Bound(1,2);
    println!("multiply{:}", b1.1 * b1.0);
}


fn unit_struct()  {
    #[derive(Debug)]
    struct OnePunch;
    let o = OnePunch;
    println!("o: {:?}", o);
}






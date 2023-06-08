fn main()  {
    
}

struct GrayScaleMap {
    pixels: vec<u8>,
    size: (usize, usize)
}

fn new_map(size:(usize, usize), pixels: vec<u8>) -> GrayScaleMap{
    assert_eq!(pixels.len, size.0 * size1);
    GrayScaleMap{ pixels, size };// 局部变量名和名字相同，省略
}

fn t1()  {
    let width = 1024;
    let height = 576;
    let image = GrayScaleMap{
        pixels: vec![0;width * height],
        size: (width, height)
    }    
}

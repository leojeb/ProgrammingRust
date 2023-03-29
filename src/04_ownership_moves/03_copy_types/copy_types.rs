fn t1() {
    let string1 = "somnambulance".to_string();
    let string2 = string1;
    let num1: i32 = 36;
    let num2 = num1;

    // 在处理 Copy 类型时，是复制值而不是 move，标准的 Copy 类型包括了所有的
    // 整数，浮点数，char，bool 类型以及大小固定的数组或者 tuple，或者说可以按 bit 复制的类型都能被 Copy
}

fn copy_struct() {
    struct Label {
        number: u32,
    }

    fn print(l: Label) {
        println!("STAMP: {}", l.number);
    }

    let l = Label { number: 3 };
    print(l);
    println!("My label number is: {}", l.number); // 失败

    // 一般来说结构体不能复制,如果结构体里面全部是可以复制的基本类型(如上所述)
    // 则可以通过加上 #[derive(Copy, Clone)]来让结构体可以复制
    #[derive(Copy, Clone)]
    struct Label {
        number: u32,
    }
    fn print(l: Lable) {
        println!("STAMP: {}", l);
    }
    let l = Label { number: 3};
    print(l);
    println!("{}", l.number);

    // 但是如果结构体内部字段不支持复制,则还是不能复制结构体
    /*  譬如将上面的Label替换成:
        struct Label {
            name: String,
        }
    */
    

}


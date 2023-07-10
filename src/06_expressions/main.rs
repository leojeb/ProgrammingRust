// use std::io;
// use std::cmp::Ordering;

// fn show_files() -> io::Result<()> {
//     let mut v = vec![];
//     //  函数不能在函数体里直接访问变量v从而形成闭包，
//     fn cmp_by_timestamp_then_name(a: &FileInfo, b: &FileInfo) -> Ordering {
//         a.timestamp.cmp(&b.timestamp)
//             // first, compare timestamps
//             .reverse()
//             // newest file first
//             .then(a.path.cmp(&b.path)) // compare paths to break ties

//     }
//     v.sort_by(cmp_by_timestamp_then_name);
// }

fn t1() {
    // rust中 表达式可以是有（结果）值的，可以当做值来用，例如表达式赋值，表达式传参，如下伪代码
    // let status =
    //     if cpu.temperature <= MAX_TEMP {
    //         HttpStatus: Ok
    //     } else {
    //         HttpStatus: ServerError
    //     }
}

fn t2() {
    let a = 1;  
    let b = match a {
        1 => println!("ok1"),
        2 => println!("ok2"),
        _ => println!("ok"),
    };
    // println!("{}", b) error  b = () and cannot be printed
}

fn if_let() {
    /*
       show_anti_robot_check()的返回值是Option<T>, 通过if let可以方便的处理这种情况
    */
    // if let Err(err) = show_anti_robot_check() {
    //     log_robot_attempt(err);
    // } else {
    //     session.mark_as_human();
    // }
}

fn loops() {
    //  rust有四种循环
    while false {
        println!("1")
    }

    let a = &1;
    while let a = Some(1) {
        println!("1");
        break;
    }

    loop {
        println!("1");
        break;
    }

    // for pattern in iterable {
    //     println!(1)
    // }

    let mut v1 = vec!["1".to_string(), "2".to_string(), "3".to_string()];
    // for i in v1 {
    //     println!("{}", i);
    // }
    // println!("{}", v1.len()); //error v1 已经被for拿走了（谁把它拿走了）
    // for循环拿走变量，这很不方便，所以用引用
    for rs in &v1 {
        println!("String {:?} is at address {:p}.", *rs, rs);
    }
    //  iterating over a mut ref, then rs will also be mut ref pointing to each element in the collection
    for rs in &mut v1 {
        // the type of rs is &mut String
        rs.push('\n'); // add a newline to each string
    }

    // break, continue, 和平常的一样，不同的是break后面可以跟变量，作为循环的返回值，当然各个break后面的变量必须是类型相同的
    let loop_res = for rs in &v1 {
        if *rs == "1".to_string() {
            // break rs // can only break with a value inside `loop` or breakable block
            println!("判断成功");
            break;
        }
    };

    // 给循环加标签
    'search: for i in &v1 {
        for a in 1..20 {
            println!("inner loop");
            break 'search 20; // break可以同时添加标签和返回值
            continue 'search; // continue可以跟标签
        }
    }

    // 对于不会正常return的函数， 他的返回值是！
    fn serve_forever(socket: ServerSocket, handler: ServerHandler) -> ! {
        socket.listen();
        loop {
            let s = socket.accept();
            handler.handle(s);
        }
    }
}

fn funtion_and_method_calls() {
    // Vec::new() :: 类似于类的静态方法
}

fn fields_and_elements() {
    
    /*
    game.black_pawns    struct field
    coords.1            tuple element
    a[i]                array element

    game.black_pawns = 0x00ff0000_00000000_u64;
    coords.1 = 0;
    pieces[2] = Some(Piece::new(Black, Knight, coords));
    
    get slice from an array
    let mut s1 = &arr1[start..end]; 这里arr1可以是array, slice或者vector
    ..  ..end   start..     start..end  ..=b    a..=b
    s1 = &arr1[..]
     */
    
}

fn reference_operator()  {

    
}

fn type_casts()  {
    let x = 17;
    let index = x as usize;
    
}

fn closures()  {
    let x = 1;
    let is_even = |x: u32| -> bool { x % 2 == 0 };
    
    let y: u32 = 0;
    let is_odd = |x| x % 2 != 0;
}



fn main() {
    t2();
    loops();
}

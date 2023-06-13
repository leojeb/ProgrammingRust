
fn get_weather(location: &str) -> Result<WeatherReport, io::Error> {

}


// Results
fn catching_errors()  {
    // 这是rust的try/catch
    match get_weather("hometown") {
        Ok(report) => {
            display_weather(hometown, &report);
        }
        Err(err) => {
            println!("error querying the weather at {}: {}", hometown, err);
            schedule_retry();
        }
    }
    // match 来捕获错误比较繁琐, Result<T,E>类有很多方法可以方便的做判断
    /*
        result.is_ok(), result.is_err() 返回bool， 告诉返回值是否是ok或者是否是error

        result.ok() 将成功值作为Option<T>返回，如果确实成功了则返回 Some(T), 否则返回None. 舍弃error值

        result.err() 将error值作为Option<E>返回，有err就是Some(error), 没有就是None

        result.unwrap_or(fallback) 
        Returns the success value, if result is a success result. Otherwise, it returns
        fallback , discarding the error value.

        result.unwrap_or_else(fallback_fn)
        This is the same, but instead of passing a fallback value directly, you pass a func‐
        tion or closure. This is for cases where it would be wasteful to compute a fallback
        value if you’re not going to use it. The fallback_fn is called only if we have an
        error result.

        result.unwrap()
        Also returns the success value, if result is a success result. However, if result is
        an error result, this method panics. This method has its uses; we’ll talk more
        about it later.

        result.expect(msg)
        This the same as .unwrap() , but lets you provide a message that it prints in case
        of panic.

        result.as_ref()
        Converts a Result<T, E> to a Result<&T, &E> .

        result.as_mut()
        This is the same, but borrows a mutable reference. The return type is
        Result<&mut T, &mut E> .
     */

    const DEFAULT_WEATHER: WeatherReport = WeatherReport::Sunny(72);
    let report =  get_weather("北京").unwrap_or(DEFAULT_WEATHER);
    
    let result = get_weather("a");
    let is_ok = result.as_ref().is_ok(); // as_ref避免result被is_ok()获取所有权
    

}

fn type_aliases()  {
    pub type Result<T> = result::Result<T, Error>;
    fn a1() -> Result<T> {} 
    
}

fn printing_errors() {
    let err = "112";
    println!("printing the err: {}", err);
    println!("printing the err: {}", err.to_string());
    println!("printing the err: {}", err.source());

}

/*
    best practice！！
    for print err with all the available information
 */ 
use std::error::Error;
use std::io::{Write, stderr};
fn print_error()  {
    let _ = writeln!(stderr(), "error: {}", err);
    while let Some(source) = err.source() {
        let _ = writeln!(stderr(), "caused by: {}", source);
        err = source;
    }
}

fn propagating_errors()  {
    // ? 成功返回WeatherReport类型值，失败直接return error，? 只能在返回值是Result类型的函数上用
    let weather = get_weather("武汉")?; 
    let ok_value = get_weather("上海").as_ref().ok()?;
    // ? 也可以用在返回值类型是Option的函数上，此时如果是函数最终返回的是None则直接return
}

fn working_with_multiple_error_types() {
    fn read_numbers(file: &mut dyn BufRead) -> Result<Vec<i64>, io::Error> {
        let mut numbers = vec![];
        for line_result in file.lines() {
            let line = line_result?;
            numbers.push(line.parse()?);
        }
        Ok(numbers)
    }
    // 上述函数有几处加了？，？会隐式的把对应函数返回的错误类型转换成我们返回值里的io::Error类型，
    // 加入有的错误类型没有实现io::Error,那么？这里就会编译报错了
    /*
        解决办法:
        1. 可以自定义错误类，实现io::Error
        2. 可以使用自带的基础错误类型，可以表示任何错误

     */
    type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
    type GenericResult<T> = Result<T, GenericError>;
    fn a() -> GenericResult<T> {
        
    }
    // 新建一个err并转换成GenericError类型
    let new_err = io::Error::new(
        io::ErrorKind::Other, "timed out"); 
    return Err(GenericError::from(new_err));
    // 第二种做法，错误类型太宽泛，按如下做法来判断返回值是否为某一特定错误类型
    match do_something() {
        Ok(()) => return Ok(()),
        Err(err) => {
            if let Some(mse) = err::downcast_ref::<MissingSemicolonError>() {
                // do something
                println!("try again: {}")
            }
            return Err(err);
        }
    }
    
}

fn dealing_with_errors_that_cannot_happen()  {
    "111111111111111111111111".parse::<u64>().unwrap();
    // 成功返回成功值，不成功直接panic，理论上讲不会发生错误，但是一旦发生错误说明有大问题hh，应该panic
    "222222222222".parse::<u64>().expect("a simple parse operation failed unexpectedly!!!");
    // 用expect加一个文字说明
}

fn ignoring_errors()  {
    let _ = error_returning_fn(); // ignore the error by using a _ to receive it.
    
}

fn handle_errs_in_main()  {
    // for tiny project, using .expect() to handle the error
    
    if let Err(err) = do_something() {
        print_error(&err); // using print_error fn mentioned before
        std::process::exit(1);
    }
}

fn declare_a_custom_error_type() {
    #[derive(Debug, Clone)]
    pub struct JsonError {
        pub msg: String,
        pub line: usize,
        pub column: usize,

    }

    use std::fmt;
    impl fmt::Display for JsonError {
        fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            write!(f, "{} ({}:{})", self.message, self.line, self.column)
        }
    }
    impl std::error::Error for JsonError { }

    /*
    上面这么繁琐的工作都可以通过使用 thiserror crate来方便的替代
     */
    use thiserror::Error;

    #[derive(Error, Debug)]
    #[error("{message:} ({line:}, {column})")]
    pub struct JsonErr{
        message: String,
        line: usize,
        column: usize,
    }
    // The #[derive(Error)] directive tells thiserror to generate the code shown earlier,
    // which can save a lot of time and effort.
}

fn main()  {
    
}
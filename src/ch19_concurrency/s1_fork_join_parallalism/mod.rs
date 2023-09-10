use std::thread::spawn;

/**
 * 创建线程和等待
 */
#[test]
pub fn create_threads_and_wait() {
    spawn(|| {
        println!("这是个一次性闭包函数{:}", 2333);
    });
}
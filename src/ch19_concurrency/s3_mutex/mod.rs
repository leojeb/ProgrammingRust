use std::io;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;

type PlayerId = u32;
const GAME_SIZE: usize = 8;

type WaitingList = Vec<PlayerId>;

struct FernEmpireApp {
    waiting_list: Mutex<WaitingList>,
}

impl FernEmpireApp {
    fn join_waiting_list(&self, player: PlayerId) {
        let mut guard = self.waiting_list.lock().unwrap();
        *guard = vec![];
        
        /*
            1. 虽然是&self不可变引用，但是guard却可以改变waiting_list的值
            因为互斥锁Mutex为我们提供了内部数据WaitingList的独占访问，本身它就是干这个的
            Mutex会动态地强制执行独占访问，通常由编译器在编译时静态完成
         */ 
        
        /*
            2. 死锁
            如下一行代码会造成死锁, 因为互斥锁被上一个语句占有了，此时这个线程又想拿到一个锁，却发现锁已经被拿了，虽然其实是被自己拿的
            但是还是认为已经被锁住了，就会一直卡在这里造成死锁，这与java中一般的两个线程互相死锁有些不同
         */
        // let mut guard1 = self.waiting_list.lock().unwrap(); 死锁
        /*
            3. 有毒的锁
            如果在锁住状态下程序panic了，则后续任何获取锁的语句都会panic，试想
            比如锁里本来要更新三个字段，但是panic了，只更新了一个，后续的程序获取锁时就应该panic否则很容易出现拿到错误的数据
            但是也可以强制访问有毒的互斥体，使用PoisonError::into_inner()
         */
        /**
         * 通道（channel）是多消费者单生产者的，无法做到多线程同时获取一个通道里的内容，将通道用作共享工作列表
         */
        guard.push(player);
        if guard.len() == GAME_SIZE {
            let players = guard.split_off(0);
            // drop(guard); 手动移除锁，这样下面的start_game过程就处于无保护状态的多线程中
            // self.start_game(players);
        }
        
    }

}

#[test]
pub fn t1() {
    /*
        Arc::new()创建一个多线程安全共享的对象，而Mutex可以方便地创建跨线程共享的可变数据
        Mutex::new()虽然看起来像Arc::new() / Box::new()， 但是与后两者不同的是，它是不会在进行堆分配的
     */
    let app = Arc::new(FernEmpireApp {
       waiting_list: Mutex::new(vec![])
    });
}

#[test]
pub fn rw_lock() {
    /*
        读写锁
        互斥锁不管用户是准备拿去读还是写，反正独占。读写锁则是可以多读或一写，和rust的owner理念一致。
        详细点说，在任何给定时刻，受保护的数据可以有一个writer或者有多个reader，写锁对于其他 读写锁 都是独占的，读锁不会独占
     */
    struct AppConfig(u32);
    struct TestRWLock{
        waiting_list: RwLock<WaitingList>,
        config: RwLock<AppConfig>,
    }
    impl TestRWLock {
        fn a(&self) -> u32 {
            let config_guard = self.config.read().unwrap();
            config_guard.0
        }

        fn b(&self) -> io::Result<()> {
            let new_config = AppConfig(0);
            let mut config_write_guard = self.config.write().unwrap();
            *config_write_guard = new_config;
            Ok(())
        }
    }
    let t1 = TestRWLock{
        waiting_list: vec![].into(),
        config: AppConfig(2).into()
    };
    println!("t1:   {:}", t1.a());
    let _ = t1.b();
    println!("t1:    {:}", t1.a());
}
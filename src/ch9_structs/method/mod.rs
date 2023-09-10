use std::{rc::Rc, io, f32::consts::E};

#[test]
fn test()  {
    // methods();
    pointers();
    lifecycle();

}

fn return_err() -> Result<String, io::Error> {
    todo!()
}

fn t1() {
    match return_err() {
        Ok(v) => Ok(v),
        Err(e) => {
            Err(e.downcast::<io::Error>())
        }
    };
}
pub struct Coordinate(pub i64, pub i64);
pub struct Queue<T> {
    pub older_q: Vec<T>,
    pub newer_q: Vec<T>
}
impl<T> Queue<T> {
    
    const Zero: Coordinate = Coordinate(1,2);
    /*
        不包含self的叫作类型方法, 类型方法用::访问
     */
    pub fn new() -> Self {
        return Queue {
            older_q: vec![],
            newer_q: vec![],
        }
    }
    /*
        包含self的叫作实例方法, 用.访问
     */
    pub fn push(&mut self, ele: T) -> &mut Self {
        self.newer_q.push(ele);
        return self;
    }

    pub fn pop(&mut self) -> Option<T> {
        return self.older_q.pop()
    }
    
    pub fn rc_fn(self: Rc<Self>) {
        println!("rc called");
    }
    pub fn get(&self) -> &Self {
        return self;
    }

    pub fn to_owned(self) -> Self {
        return self;
    }
}

fn methods()  {
    
    let mut q = Queue{
        older_q: vec![],
        newer_q: vec![]
    };
    let res = q.push(1);
    let ele = res.pop();
    println!("older_q {:?}", res.older_q);
    println!("newer_q {:?}", res.newer_q);
    let sel = q.get();
    println!("self.newer {:?}", sel.newer_q);
    // println!("ele {:}", ele.unwrap());
    let own = q.to_owned();
    // q.get(); q already moved
    own.get();
    // assert_eq!(q, None);
}

/**
 * 智能指针
 * 智能指针诸如 Box<T>，Rc<T> 以及 Arc<T> 都可以调用 self 类型的方法
 */
fn pointers()  {
    let mut bq = Box::new(Queue::<u8>::new());
    bq.push(2); // 直接调用该类型的实例或静态方法
    
    let mut rcq = Rc::new(Queue::<i8>::new());
    rcq.rc_fn();
    let c1 = Queue::<u16>::Zero;


}

/// 生命周期演示
#[test]
fn lifecycle() {
    pub struct Extrema<'elt> {
        biggest: &'elt i8,
        smallest: &'elt i8,
    }
    
    fn find_extrema<'s>(slice: &'s [i8]) -> Extrema<'s> {
        let mut biggest = &slice[0];
        let mut smallest = &slice[0];
        for i in 1..slice.len() {
            if slice[i] < *smallest { smallest = &slice[i]};
            if slice[i] > *biggest { biggest = &slice[i]};
        }
        println!("biggest {:}", biggest);
        println!("smallest {:}", smallest);
        Extrema {
            biggest,
            smallest
        }
    }
    let s = &vec![1,2,3];
    let ext = find_extrema(s);
    println!("ext biggest{:}", ext.biggest);
    println!("ext smallest{:}", ext.smallest);
}

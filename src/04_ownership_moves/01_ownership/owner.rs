fn t1 () {
    let mut padovan = vec![1,2,3];
    for i in 3..10{
        let next = padovan[i-3] + padovan[i-2];
        padovan.push(next);
    }
    println!("P(1..10) = {:?}", padovan)
}

fn t2 () {
    let point = Box::new((0.1,0.2));
    let label = format!("{:?}", point);
    assert_eq!(label, "(0.1, 0.2)")
    
}

fn t3 () {
    // Struct owns their fields
    struct Person { name: String, birth: i32}
    let mut composers = Vec::new();
    composers.push(Person{
        name: "Palestrina".to_string(),
        birth: 12
    });

    
}



fn main () {
    t2()
    
    /*
    在ownership的规则下如何实现更多的灵活性，rust有如下做法：
        1. 将值从一个owner移动到另一个owner里
        2. 简单类型，例如int， float， char等不包含在ownership的规则里， 它们叫做copy types
        3. 利用rc和arc可以计算值有多少个引用， 从而决定何时释放内存
        4. 使用引用，引用是non-owning的指针， 生命周期有限
     */ 
}

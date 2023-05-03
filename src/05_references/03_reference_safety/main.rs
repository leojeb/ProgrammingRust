fn t1 () {
    {
        let r;
        {
            let x = 1;
            r = &x;
        }
        assert_eq!(*r, 1); // bad: reads memory `x` used to occupy, x已经无了. 
    }
}

fn t2 () {
    
}

fn t3 () {
    
}

fn t4 () {
    
}

fn t5 () {
    
}

fn t6 () {
    
}

fn main () {
    
}

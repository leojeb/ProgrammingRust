use std::ops::Index;

#[test]
fn t1()  {
    print_str("a3[bc]".to_owned());
    
}



fn print_str(s: String) -> String {
    let chars: Vec<char> = s.chars().collect::<Vec<char>>();
    
    for i in 0..s.len() {
        let c = s[i];
        match s[i] {
            'a'..='z' => {
                print!("{:}",c);
            },
            '1'..='9' => {
                
                let l_bracket_idx = s.find("[").unwrap();
                let r_bracket_idx = s.rfind("]").unwrap();
                let cnt: u8 =(c as u8) - ('0' as u8);
                println!("cnt {:}", cnt);
                for i in 0..cnt{
                    print_str(s[l_bracket_idx+1..r_bracket_idx].to_owned());
                }
                i += 
            }
            _ => ()
        }
        
    }
    "".to_owned()
}
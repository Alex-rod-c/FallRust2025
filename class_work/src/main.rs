fn modify_clone(s: &String) -> String{
    let mut cloned = s.clone();
    cloned.push_str("world!");
    cloned
}

fn main(){
    let s = String::from("Hello, ");
    let modified = modify_clone(&s);
    println!("Orginal: {}", s);
    println!("Modified: {}", modified);
}



/*fn con_string(s1: &String, s2: &String) -> String
{
    let mut result = String::new();
    result.push_str(s1);
    result.push_str(s2);
    result
}



fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let result = con_string(&s1, &s2);
    println!("{}", result);
 }
\*


fn con_string(s1: &String, s2: &String) -> String
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


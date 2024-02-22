// write a function that concatenates two strings
fn concat_strings(s1: &str, s2: &str) -> String {
    let mut result = String::from(s1);
    result.push_str(s2);
    result
}


// write a hello world main function
fn main() {
    println!("Hello, world!");
    let s1 = "Hello, ";
    let s2 = "world!";
    println!("{}", concat_strings(s1, s2));
}

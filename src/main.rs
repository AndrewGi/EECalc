mod scanner;
mod parser;
fn main() {
    println!("Hello, world!");
    let mut x = parser::Parser::new("10na * 10gv");
    println!("{}", x.next_value().unwrap());
    println!("{}", x.next_operator().unwrap());
    println!("{}", x.next_value().unwrap());
}

#[macro_use]
extern crate lazy_static;
mod si;
mod scanner;
mod parser;
fn main() {
    println!("rust calc\
    ");
    let mut x = scanner::Scanner::new("10na * 10gv");
    println!("{:?}", x.get_cursor().next_value());
    //println!("{}", x.next_value().unwrap());
}

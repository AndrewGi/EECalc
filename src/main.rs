#[macro_use]
extern crate lazy_static;
mod si;
mod scanner;
mod parser;
fn main() {
    println!("rust calc\
    ");
    let mut _x = parser::Parser::new("10na * 10gv");
    //println!("{}", x.next_value().unwrap());
}

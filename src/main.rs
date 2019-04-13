mod scanner;
mod parser;
mod si;
fn main() {
    println!("rust calc");
    let mut x = parser::Parser::new("10na * 10gv");
    //println!("{}", x.next_value().unwrap());
}

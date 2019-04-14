mod si;
mod scanner;
mod parser;
fn main() {
    println!("rust calc");
    let mut x = parser::Parser::new("10na * 10gv");
    //println!("{}", x.next_value().unwrap());
}

#[macro_use]
extern crate optional;

#[optional]
#[derive(Debug)]
struct Test {
    n: u32,
}
fn main() {
    println!("{:?}", Test::optional());
}

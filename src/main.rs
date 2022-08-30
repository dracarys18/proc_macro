#[macro_use]
extern crate builder;

#[derive(Default, Debug, Builder)]
struct Test {
    #[def = "0"]
    n: u32,
}
fn main() {
    println!("{:?}", Test::default());
}

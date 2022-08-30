use promacro::{no_unwrap, optional, reference, wtf, TestBuilder};

wtf!();

#[optional]
#[derive(Default, Debug)]
struct Test {
    n: u32,
}
fn main() {
    println!("{:?}", Test::default());
}

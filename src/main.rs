use promacro::{no_unwrap, reference, wtf, TestBuilder};

wtf!();

#[derive(Debug, TestBuilder)]
struct Test {
    n: Vec<i32>,
}
fn main() {
    let mut builder = Test::builder();
    builder.with_n(vec![3]);
    println!("{:?}", builder.build());
}

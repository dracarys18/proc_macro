use promacro::{context, no_unwrap, wtf};

wtf!();

#[context]
pub fn lol(n: i32, f: &[i32], l: i32) {
    println!("{:?}", n);
}

#[no_unwrap]
pub fn kek() {
    let n: Option<i32> = None;
    n.unwrap();
}

fn main() {
    lol(1, &[1, 2], 2);
    println!("{}", lmao());
}
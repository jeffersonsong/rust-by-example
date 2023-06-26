use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let int = 30;
    let num: Number = 30.into();
    //let num = Number::from(30);
    println!("My number is {:?}", num);
}

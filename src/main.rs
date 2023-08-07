use matrix::Vector;

fn main() {
    let a = Vector([1,2,3]);
    let b = Vector([1,2,3]);
    let r = a + b;
    println!("{:?}", r)
}
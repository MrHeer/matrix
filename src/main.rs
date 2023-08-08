use matrix::Vector;

fn main() {
    let a = Vector([8.218, -9.341]);
    let b = Vector([-1.129, 2.111]);
    let r = &a + &b;

    println!("{} + {} = {}", a, b, r);
}

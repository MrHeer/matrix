use matrix::vector;

fn main() {
    let a = vector([8.218, -9.341]);
    let b = vector([-1.129, 2.111]);
    let r = (a + b).round(3);

    println!("{} + {} = {}", a, b, r);
}

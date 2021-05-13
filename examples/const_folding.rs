fn main() {

    let x = math();
    println!("{}", x);
}

fn math() -> usize {
    let x = 5;
    let y = 10;

    let z = 5 * x * y;

    let a = 1234 + z;

    return a;
}
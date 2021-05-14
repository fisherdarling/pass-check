fn main() {

    let x = "10".parse().unwrap();

    let mut total = 0;
    for i in 1..x {
        total += i;
    }

    println!("{}", x);
}
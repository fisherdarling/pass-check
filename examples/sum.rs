fn main() {

    // let x = "10"

    let x = sum("10".parse().unwrap());
    println!("{}", x);
}

fn sum(n: usize) -> usize {
    let mut total = 0;
    for x in 1..n {
        total += x;
    }
    total
}
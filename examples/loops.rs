pub fn main() {
    let mut x = [0; 10000];
    for i in 0..10000 {
        x[i] += 1;
    }
    for i in 0..10000 {
        println!("{}", x[i])
    }
}

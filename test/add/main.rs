fn main() {
    let mut lines = std::io::stdin().lines();
    let a = lines.next().unwrap().unwrap();
    let a: i32 = a.parse().unwrap();

    let b = lines.next().unwrap().unwrap();
    let b: i32 = b.parse().unwrap();

    println!("{}", a + b);
}

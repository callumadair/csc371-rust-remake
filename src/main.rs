fn main() {
    println!("Hello, world!");
    let n = add_one(2);
    println!("{}", n);
}

fn add_one(n: u32) -> u32 {
    let n: u32 = n + 1;
    return n;
}

fn main() {
    let (mut a, mut b) = (1u64, 2u64);
    let mut sum = 2;
    loop {
        let c = a + b;
        if c > 4_000_000 {
            break;
        }
        if c % 2 == 0 {
            sum += c;
        }
        (a, b) = (b, c);
    }
    println!("{sum}");
}

fn main() {
    let sum_square : u64 = (1..=100u64).map(|x| x.pow(2)).sum();
    let square_sum : u64 = (1..=100u64).sum::<u64>().pow(2);
    let res = square_sum - sum_square;

    println!("{res}");
}

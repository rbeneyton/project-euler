fn main() {
    'cand: for r in 1.. {
        for n in 2..=20 {
            let (_, rem) = num_integer::div_rem(r, n);
            if rem != 0 {
                continue 'cand;
            }
        }
        println!("{r}");
        return;
    }
}

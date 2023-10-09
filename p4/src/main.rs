fn main() {
    let (mut res, mut res_a, mut res_b) = (0, 0, 0);
    for a in 100..999 {
        for b in 100..999 {
            let p = a * b;
            if p < res { continue; }
            if p < 100_000 { continue; }
            debug_assert!(p < 1_000_000);
            let w = p;
            let (c1, w) = num_integer::div_rem(w, 100_000);
            let (c2, w) = num_integer::div_rem(w, 10_000);
            let (c3, w) = num_integer::div_rem(w, 1_000);
            let (c4, w) = num_integer::div_rem(w, 100);
            let (c5, w) = num_integer::div_rem(w, 10);
            let c6 = w;
            debug_assert!(c6 < 10);
            if c1 == c6 && c2 == c5 && c3 == c4 && p > res {
                res = p;
                res_a = a;
                res_b = b;
            }
        }
    }
    println!("{res} = {res_a}x{res_b}");
}

pub fn is_prime(n: u64) -> bool {
    if n < 4 {
        n > 1
    } else if n % 2 == 0 || n % 3 == 0 {
        false
    } else {
        let max_p = (n as f64).sqrt().ceil() as u64;
        match (5..=max_p).step_by(6).find(|p| n % p == 0 || n % (p+2) == 0) {
            Some(_) => false,
            None => true
        }
    }
}

pub struct Prime {
    curr: u64,
    next: u64,
}

impl Prime {
    pub fn new() -> Prime {
        Prime {
            curr: 2,
            next: 3,
        }
    }
}

impl Iterator for Prime {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let prime = self.curr;
        self.curr = self.next;
        loop {
            self.next += match self.next%6 {
                1 => 4,
                _ => 2,
            };
            if is_prime(self.next) {
                break;
            }
        }
        Some(prime)
    }
}

fn max_prime(a: u64) -> u64 {
    let mut primes = rustc_hash::FxHashSet::default();
    let mut w = a;
    for prime in Prime::new() {
        let (n, rem) = num_integer::div_rem(w, prime);
        if rem == 0 {
            w = n;
            primes.insert(prime);
            if w == 1 {
                dbg!(primes);
                return prime;
            }
        }
    }
    unreachable!();
}

fn main() {
    let r = max_prime(600851475143u64);
    println!("{r}");
}

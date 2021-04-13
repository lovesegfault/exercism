pub fn factors(mut n: u64) -> Vec<u64> {
    let mut primes = Vec::<u64>::new();
    let mut candidate = 2;
    while n > 1 {
        while n % candidate == 0 {
            primes.push(candidate);
            n /= candidate;
        }
        candidate += 1;
    }
    primes
}

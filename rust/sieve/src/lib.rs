pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let upper_bound = upper_bound as usize;
    let prime_limit = {
        let x = upper_bound as f64;
        if x <= 5.0 {
            10.0
        } else {
            x * x.ln() + x * x.ln().ln()
        }
    }
    .ceil() as usize;

    let mut sieve = vec![true; upper_bound + 1];
    let mut primes = Vec::with_capacity(prime_limit);

    for i in 2..=upper_bound {
        if sieve[i] {
            primes.push(i as u64);
            for j in (i..=upper_bound).step_by(i) {
                sieve[j] = false;
            }
        }
    }

    primes
}

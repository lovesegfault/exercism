pub fn nth(n: u32) -> u32 {
    let n = n as usize + 1;

    let x = f64::from(n as u32);
    let limit = if x <= 5.0 {
        10.0
    } else {
        x * x.ln() + x * x.ln().ln()
    };
    let limit = limit.ceil() as usize;

    let mut sieve = vec![true; limit];
    let mut count = 0;

    for i in 2..limit {
        if sieve[i] {
            count += 1;

            if count == n {
                return i as u32;
            }

            for j in ((i * 2)..limit).step_by(i) {
                sieve[j] = false;
            }
        }
    }

    unreachable!()
}

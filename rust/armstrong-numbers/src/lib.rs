pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits = Vec::new();

    let mut c = num;
    while c != 0 {
        digits.push(c % 10);
        c /= 10;
    }

    let num_len = digits.len() as u32;
    let sum: u32 = digits.into_iter().map(|d| d.pow(num_len)).sum();

    sum == num
}

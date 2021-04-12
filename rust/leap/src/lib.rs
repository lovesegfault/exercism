/// Returns truw if the numerator (num) is divisible by the denominator (den).
#[inline(always)]
fn is_div(num: u64, den: u64) -> bool {
    num % den == 0
}

pub fn is_leap_year(year: u64) -> bool {
    let table = (is_div(year, 4), is_div(year, 100), is_div(year, 400));
    match table {
        (false, _, _) => false,
        (true, true, false) => false,
        (true, true, true) => true,
        (true, false, _) => true,
    }
}

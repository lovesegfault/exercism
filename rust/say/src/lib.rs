pub fn encode(n: u64) -> String {
    match n {
        0 => String::from("zero"),
        1 => String::from("one"),
        2 => String::from("two"),
        3 => String::from("three"),
        4 => String::from("four"),
        5 => String::from("five"),
        6 => String::from("six"),
        7 => String::from("seven"),
        8 => String::from("eight"),
        9 => String::from("nine"),
        10 => String::from("ten"),
        11 => String::from("eleven"),
        12 => String::from("twelve"),
        13 => String::from("thirteen"),
        14 => String::from("fourteen"),
        15 => String::from("fifteen"),
        16 => String::from("sixteen"),
        17 => String::from("seventeen"),
        18 => String::from("eighteen"),
        19 => String::from("nineteen"),
        20 => String::from("twenty"),
        30 => String::from("thirty"),
        40 => String::from("forty"),
        50 => String::from("fifty"),
        60 => String::from("sixty"),
        70 => String::from("seventy"),
        80 => String::from("eighty"),
        90 => String::from("ninety"),
        20..=99 => format!("{}-{}", encode(10 * (n / 10)), encode(n % 10)),
        100..=999 => format!("{} hundred {}", encode(n / 100), encode(n % 100)),
        1000..=999_999 => format!("{} thousand {}", encode(n / 1000), encode(n % 1000)),
        1_000_000..=999_999_999 => format!(
            "{} million {}",
            encode(n / 1_000_000),
            encode(n % 1_000_000)
        ),
        1_000_000_000..=999_999_999_999 => format!(
            "{} billion {}",
            encode(n / 1_000_000_000),
            encode(n % 1_000_000_000)
        ),
        1_000_000_000_000..=999_999_999_999_999 => format!(
            "{} trillion {}",
            encode(n / 1_000_000_000_000),
            encode(n % 1_000_000_000_000)
        ),
        1_000_000_000_000_000..=999_999_999_999_999_999 => format!(
            "{} quadrillion {}",
            encode(n / 1_000_000_000_000_000),
            encode(n % 1_000_000_000_000_000)
        ),
        1_000_000_000_000_000_000..=u64::MAX => format!(
            "{} quintillion {}",
            encode(n / 1_000_000_000_000_000_000),
            encode(n % 1_000_000_000_000_000_000)
        ),
    }
    .replace(" zero", "")
}

pub fn raindrops(n: u32) -> String {
    let is_factor = |factor: u32| n % factor == 0;
    let mut out: String = String::with_capacity("PlingPlangPlong".len());

    if is_factor(3) {
        out.push_str("Pling");
    }
    if is_factor(5) {
        out.push_str("Plang");
    }
    if is_factor(7) {
        out.push_str("Plong")
    }

    if out.is_empty() {
        out.push_str(&n.to_string());
    }

    out
}

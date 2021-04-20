use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

fn aliquot_sum(n: u64) -> u64 {
    let limit = (n / 2) + 1;
    (1..limit).filter(|i| n % i == 0).sum()
}

pub fn classify(num: u64) -> Option<Classification> {
    match num {
        0 => None,
        n => match n.cmp(&aliquot_sum(n)) {
            Ordering::Equal => Some(Classification::Perfect),
            Ordering::Greater => Some(Classification::Deficient),
            Ordering::Less => Some(Classification::Abundant),
        },
    }
}

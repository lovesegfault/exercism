use chrono::{DateTime, Duration, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    // ideally this would be a const, but chrono::Duration::seconds isn't a const fn
    let gigasecond = Duration::seconds(10_i64.pow(9));

    start + gigasecond
}

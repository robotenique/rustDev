extern crate chrono;
use chrono::*;


// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    // The Duration is from chrono crate, not to confuse with std::time:Duration
    let dur = Duration::seconds(1e9 as i64);
    start + dur
}

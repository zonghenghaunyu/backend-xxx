pub mod n_17;
pub mod n_216;
pub mod n_77;
pub mod n_39;
pub mod n_131;
pub mod n_93;

extern crate chrono;
use chrono::{DateTime, TimeZone, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    Utc.timestamp_millis(start.timestamp_millis() + 1000000000000)
    //    unimplemented!("What time is a gigasecond later than {}", start);
}

#[cfg(test)]
mod test {
    use chrono::TimeZone;

    use super::*;
    #[test]
    fn test_empty() {
        let start_date = Utc.ymd(2011, 4, 25).and_hms(0, 0, 0);

        assert_eq!(after(start_date), Utc.ymd(2043, 1, 1).and_hms(1, 46, 40));
    }
}

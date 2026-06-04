use chrono::{TimeZone, Utc};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct UnixTimestamp(pub i64);

impl UnixTimestamp {
    pub fn now() -> Self {
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        UnixTimestamp(secs)
    }

    pub fn prettify(&self) -> String {
        let dt = Utc.timestamp_opt(self.0, 0).single().unwrap();
        dt.to_rfc3339()
    }

    pub fn is_due(&self) -> bool {
        let now = UnixTimestamp::now();
        self.0 <= now.0
    }
}

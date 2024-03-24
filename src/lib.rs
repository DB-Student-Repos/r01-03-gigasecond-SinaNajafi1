use time::PrimitiveDateTime as DateTime;
use std::time::Duration;

pub fn after(start: DateTime) -> DateTime {
    let _dur = start + Duration::from_secs(1000000000);
    _dur
}

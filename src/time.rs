use time::OffsetDateTime;

/// Gets the current time with an offset from UTC.
///
/// It uses the `now_local` method on the `OffsetDateTime` struct to get the current local time, and if that fails (for example, if the system clock is not set correctly), it falls back to getting the current UTC time using the `now_utc` method.
pub fn get_offset_time() -> time::OffsetDateTime {
    OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc())
}

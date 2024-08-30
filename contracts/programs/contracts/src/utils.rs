pub fn calculate_days_between_timestamps(a: i64, b: i64) -> u64 {
    let start_timestamp = if a < b { a } else { b };
    let end_timestamp = if a < b { b } else { a };
    let duration_in_seconds = end_timestamp - start_timestamp;
    let days = duration_in_seconds / (24 * 60 * 60); // Convert seconds to days
    days.abs() as u64
}

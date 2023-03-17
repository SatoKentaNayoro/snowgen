use snowflake_rs::{Epoch, SnowflakeBuilder};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[test]
fn test_epoch_system_time() {
    let mut snowflake = SnowflakeBuilder::new()
        .epoch(Epoch::SystemTime(SystemTime::now()))
        .node_id(1)
        .build()
        .unwrap();

    assert!(snowflake.next_id().is_ok());
}

#[test]
fn test_epoch_instant() {
    let mut snowflake = SnowflakeBuilder::new()
        .epoch(Epoch::Instant(std::time::Instant::now()))
        .node_id(1)
        .build()
        .unwrap();

    assert!(snowflake.next_id().is_ok());
}

#[test]
fn test_epoch_milliseconds_since_unix_epoch() {
    let now = SystemTime::now();
    let unix_epoch = now.duration_since(UNIX_EPOCH).unwrap();
    let millis_since_unix_epoch = unix_epoch.as_millis() as i64;

    let mut snowflake = SnowflakeBuilder::new()
        .epoch(Epoch::MillisecondsSinceUnixEpoch(millis_since_unix_epoch))
        .node_id(1)
        .build()
        .unwrap();

    assert!(snowflake.next_id().is_ok());
}

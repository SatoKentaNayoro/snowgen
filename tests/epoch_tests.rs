use snowgen::{Epoch, SnowgenBuilder};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn test_epoch_system_time() {
    let mut snowgen = SnowgenBuilder::new()
        .epoch(Epoch::SystemTime(SystemTime::now()))
        .node_id(1)
        .build()
        .unwrap();

    assert!(snowgen.next_id().is_ok());
}

#[test]
fn test_epoch_instant() {
    let mut snowgen = SnowgenBuilder::new()
        .epoch(Epoch::Instant(std::time::Instant::now()))
        .node_id(1)
        .build()
        .unwrap();

    assert!(snowgen.next_id().is_ok());
}

#[test]
fn test_epoch_milliseconds_since_unix_epoch() {
    let now = SystemTime::now();
    let unix_epoch = now.duration_since(UNIX_EPOCH).unwrap();
    let millis_since_unix_epoch = unix_epoch.as_millis() as i64;

    let mut snowgen = SnowgenBuilder::new()
        .epoch(Epoch::MillisecondsSinceUnixEpoch(millis_since_unix_epoch))
        .node_id(1)
        .build()
        .unwrap();

    assert!(snowgen.next_id().is_ok());
}

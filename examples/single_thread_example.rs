use snowflake_rs::{SnowflakeBuilder, Epoch};
use std::time::SystemTime;

fn main() {
    let snowflake = SnowflakeBuilder::new()
        .epoch(Epoch::SystemTime(SystemTime::UNIX_EPOCH))
        .node_id(1)
        .build()
        .unwrap();

    let mut snowflake_instance = snowflake;
    for _ in 0..10 {
        let id = snowflake_instance.next_id().unwrap();
        println!("Generated ID: {}", id);
    }
}
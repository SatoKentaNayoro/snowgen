/// This example demonstrates the use of the `snowflake_rs` crate in a single-threaded environment.
/// It creates a Snowflake instance and generates 10 unique IDs sequentially.
use snowflake_rs::{SnowflakeBuilder, Epoch};
use std::time::SystemTime;

fn main() {
    // Create a Snowflake instance with a specific epoch and node_id
    let snowflake = SnowflakeBuilder::new()
        .epoch(Epoch::SystemTime(SystemTime::UNIX_EPOCH))
        .node_id(1)
        .build()
        .unwrap();

    let mut snowflake_instance = snowflake;

    // Generate and print 10 unique IDs
    for _ in 0..10 {
        let id = snowflake_instance.next_id().unwrap();
        println!("Generated ID: {}", id);
    }
}
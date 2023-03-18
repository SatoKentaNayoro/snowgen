/// This example demonstrates the use of the `snowgen` crate in a single-threaded environment.
/// It creates a Snowflake instance and generates 10 unique IDs sequentially.
use snowgen::{Epoch, SnowgenBuilder};
use std::time::SystemTime;

fn main() {
    // Create a Snowflake instance with a specific epoch and node_id
    let snowgen = SnowgenBuilder::new()
        .epoch(Epoch::SystemTime(SystemTime::UNIX_EPOCH))
        .node_id(1)
        .build()
        .unwrap();

    let mut snowgen_instance = snowgen;

    // Generate and print 10 unique IDs
    for _ in 0..10 {
        let id = snowgen_instance.next_id().unwrap();
        println!("Generated ID: {}", id);
    }
}

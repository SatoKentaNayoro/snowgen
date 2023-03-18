/// This example demonstrates the use of the `snowgen` crate in a multi-threaded environment.
/// It creates a Snowflake instance and shares it across 10 threads, generating unique IDs in each thread.
use snowgen::{Epoch, SnowgenBuilder};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::SystemTime;

fn main() {
    // Create a Snowgen instance with a specific epoch and node_id
    let snowgen = SnowgenBuilder::new()
        .epoch(Epoch::SystemTime(SystemTime::UNIX_EPOCH))
        .node_id(1)
        .build()
        .unwrap();

    // Wrap the Snowgen instance in an Arc<Mutex> for thread-safe sharing
    let snowflake_instance = Arc::new(Mutex::new(snowgen));

    let mut handles = vec![];

    // Spawn 10 threads and generate a unique ID in each thread
    for _ in 0..10 {
        let snowgen_clone = Arc::clone(&snowflake_instance);
        let handle = thread::spawn(move || {
            let mut instance = snowgen_clone.lock().unwrap();
            let id = instance.next_id().unwrap();
            println!("Generated ID: {}", id);
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
}

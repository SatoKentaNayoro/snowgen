use snowflake_rs::{SnowflakeBuilder, Epoch};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::SystemTime;

fn main() {
    let snowflake = SnowflakeBuilder::new()
        .epoch(Epoch::SystemTime(SystemTime::UNIX_EPOCH))
        .node_id(1)
        .build()
        .unwrap();

    let snowflake_instance = Arc::new(Mutex::new(snowflake));

    let mut handles = vec![];

    for _ in 0..10 {
        let snowflake_clone = Arc::clone(&snowflake_instance);
        let handle = thread::spawn(move || {
            let mut instance = snowflake_clone.lock().unwrap();
            let id = instance.next_id().unwrap();
            println!("Generated ID: {}", id);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

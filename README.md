<p align="center">
  <a href="https://opensource.org/licenses/MIT"><img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-yellow.svg"></a>
  <a href="https://opensource.org/licenses/Apache-2.0"><img alt="License: Apache 2.0" src="https://img.shields.io/badge/License-Apache%202.0-blue.svg"></a>
</p>

Snowgen
============

`Snowgen` is a Rust-based library for generating unique, distributed IDs. It's inspired by Twitter's Snowflake ID generator and provides an efficient, thread-safe solution for generating unique IDs in distributed systems.

Features
--------

*   Customizable epoch, node ID, machine ID, and bit lengths for each component of the ID
*   Thread-safe, ensuring unique IDs even in multi-threaded environments
*   Builder pattern for easy configuration and validation

Getting Started
---------------

Add the following line to your `Cargo.toml` file under `[dependencies]`:


```toml
snowgen = "0.1.0"
```

Usage
-----


```rust
use snowgen::{SnowgenBuilder, Epoch};

fn main() {
    // Create a new Snowgen builder with default values
    let builder = SnowgenBuilder::new();

    // Customize the builder
    let snowgen = builder
        .node_id(1)
        .machine_id(2)
        .epoch(Epoch::MillisecondsSinceUnixEpoch(1615890112000))
        .timestamp_bits(41)
        .node_id_bits(5)
        .machine_id_bits(5)
        .sequence_bits(12)
        .build()
        .unwrap();

    // Generate a unique ID
    let unique_id = snowgen.next_id().unwrap();
    println!("Generated ID: {}", unique_id);
}
```

Benchmark Results
-----------------


```text
single_thread_snowgen time:   [243.83 ns 244.11 ns 244.41 ns]
                        change: [-0.3326% -0.0319% +0.2692%] (p = 0.84 > 0.05)
                        No change in performance detected.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low severe
  3 (3.00%) low mild

multi_thread_snowgen  time:   [52.552 µs 52.991 µs 53.352 µs]
                        change: [-0.8495% -0.0131% +0.7323%] (p = 0.98 > 0.05)
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) low severe
  2 (2.00%) high mild
```

License
-------

This project is licensed under the [License](LICENSE).
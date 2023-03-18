//! Builder is a module that provides a SnowflakeBuilder struct for configuring and creating a Snowflake instance.

use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use crate::snowflake::Snowflake;

/// SnowflakeBuilder is a struct for building and configuring Snowflake instances.
pub struct SnowflakeBuilder {
    node_id: i32,
    machine_id: Option<i32>,
    epoch: Epoch,
    timestamp_bits: u8,
    node_id_bits: u8,
    machine_id_bits: u8,
    sequence_bits: u8,
}

/// Epoch represents different ways to define the epoch for the Snowflake generator.
pub enum Epoch {
    SystemTime(SystemTime),
    Instant(Instant),
    MillisecondsSinceUnixEpoch(i64),
}

impl Epoch {
    /// Converts the given Epoch variant to an Instant.
    ///
    /// # Returns
    ///
    /// An Instant representing the epoch.
    fn to_instant(&self) -> Instant {
        match self {
            Epoch::SystemTime(sys_time) => {
                let duration_since_unix = sys_time.duration_since(UNIX_EPOCH).unwrap();
                Instant::now() - (SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - duration_since_unix)
            }
            Epoch::Instant(instant) => *instant,
            Epoch::MillisecondsSinceUnixEpoch(ms) => {
                let duration_since_unix = Duration::from_millis(*ms as u64);
                Instant::now() - (SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - duration_since_unix)
            }
        }
    }
}

impl SnowflakeBuilder {
    /// Create a new SnowflakeBuilder with default values.
    ///
    /// # Returns
    ///
    /// A new SnowflakeBuilder instance.
    pub fn new() -> SnowflakeBuilder {
        SnowflakeBuilder {
            node_id: 0,
            machine_id: None,
            epoch: Epoch::SystemTime(SystemTime::UNIX_EPOCH),
            timestamp_bits: 41,
            node_id_bits: 5,
            machine_id_bits: 5,
            sequence_bits: 12,
        }
    }

    /// Set the epoch for the Snowflake generator.
    ///
    /// # Arguments
    ///
    /// * `epoch` - The desired epoch value of type `Epoch`.
    ///
    /// # Returns
    ///
    /// A new SnowflakeBuilder instance with the updated epoch value.
    pub fn epoch(mut self, epoch: Epoch) -> Self {
        self.epoch = epoch;
        self
    }

    /// Set the node_id for the Snowflake generator.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The desired node_id as an i32.
    ///
    /// # Returns
    ///
    /// A new SnowflakeBuilder instance with the updated node_id value.
    pub fn node_id(mut self, node_id: i32) -> Self {
        self.node_id = node_id;
        self
    }

    /// Set the machine_id for the Snowflake generator.
    ///
    /// # Arguments
    ///
    /// * `machine_id` - The desired machine_id as an i32.
    ///
    /// # Returns
    ///
    /// A new SnowflakeBuilder instance with the updated machine_id value.
    pub fn machine_id(mut self, machine_id: i32) -> Self {
        self.machine_id = Some(machine_id);
        self
    }

    /// Set the timestamp_bits for the Snowflake generator.
    ///
    /// # Arguments
    ///
    /// * `timestamp_bits` - The desired number of bits for the timestamp as a u8.
    ///
    /// # Returns
    ///
    /// A new SnowflakeBuilder instance with the updated timestamp_bits value.
    pub fn timestamp_bits(mut self, timestamp_bits: u8) -> Self {
        self.timestamp_bits = timestamp_bits;
        self
    }

    /// Set the node_id_bits for the Snowflake generator.
    ///
    /// # Arguments
    ///
    /// * `node_id_bits` - The desired number of bits for the node_id as a u8.
    ///
    /// # Returns
    ///
    /// A new SnowflakeBuilder instance with the updated node_id_bits value.
    pub fn node_id_bits(mut self, node_id_bits: u8) -> Self {
        self.node_id_bits = node_id_bits;
        self
    }

    /// Set the machine_id_bits for the Snowflake generator.
    ///
    /// # Arguments
    ///
    /// * `machine_id_bits` - The desired number of bits for the machine_id as a u8.
    ///
    /// # Returns
    ///
    /// A new SnowflakeBuilder instance with the updated machine_id_bits value.
    pub fn machine_id_bits(mut self, machine_id_bits: u8) -> Self {
        self.machine_id_bits = machine_id_bits;
        self
    }

    /// Set the sequence_bits for the Snowflake generator.
    ///
    /// # Arguments
    ///
    /// * `sequence_bits` - The desired number of bits for the sequence as a u8.
    /// # Returns
    ///
    /// A new SnowflakeBuilder instance with the updated sequence_bits value.
    pub fn sequence_bits(mut self, sequence_bits: u8) -> Self {
        self.sequence_bits = sequence_bits;
        self
    }

    /// Build and validate the Snowflake generator.
    ///
    /// This method ensures that the provided configuration is valid, including
    /// checking the sum of bits, node_id, and machine_id ranges.
    ///
    /// # Returns
    ///
    /// A `Result` containing a new Snowflake instance if the configuration is valid,
    /// or an error message describing the issue otherwise.
    pub fn build(self) -> Result<Snowflake, &'static str> {
        //Verify that the sum of bits does not exceed 64
        if self.timestamp_bits + self.node_id_bits + self.machine_id_bits + self.sequence_bits > 64 {
            return Err("The sum of timestamp_bits, node_id_bits, machine_id_bits, and sequence_bits should not exceed 64.");
        }

        // Verify that node_id is within the valid range
        let max_node_id = (1 << self.node_id_bits) - 1;
        if self.node_id < 0 || self.node_id > max_node_id {
            return Err("Invalid node_id, it should be between 0 and the maximum node_id.");
        }

        // Verify that machine_id is within the valid range
        if let Some(machine_id) = self.machine_id {
            let max_machine_id = (1 << self.machine_id_bits) - 1;
            if machine_id < 0 || machine_id > max_machine_id {
                return Err("Invalid machine_id, it should be between 0 and the maximum machine_id.");
            }
        }

        Ok(Snowflake {
            node_id: self.node_id,
            machine_id: self.machine_id,
            epoch: self.epoch.to_instant(),
            timestamp_bits: self.timestamp_bits,
            node_id_bits: self.node_id_bits,
            machine_id_bits: self.machine_id_bits,
            sequence_bits: self.sequence_bits,
            last_timestamp: -1,
            sequence: 0,
        })
    }
}
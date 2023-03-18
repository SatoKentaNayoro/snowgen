//! Snowgen is a module that provides a struct for generating unique, distributed IDs.

use std::time::{Duration, Instant};

/// Snowgen struct represents a unique ID generator.
pub struct Snowgen {
    pub(crate) node_id: i32,
    pub(crate) machine_id: Option<i32>,
    pub(crate) epoch: Instant,
    #[allow(dead_code)]
    pub(crate) timestamp_bits: u8,
    pub(crate) node_id_bits: u8,
    pub(crate) machine_id_bits: u8,
    pub(crate) sequence_bits: u8,
    pub(crate) last_timestamp: i64,
    pub(crate) sequence: u16,
}

impl Snowgen {
    /// Generate the next unique ID.
    ///
    /// # Returns
    ///
    /// A Result containing a u64 unique ID or an error message.
    pub fn next_id(&mut self) -> Result<u64, &'static str> {
        let timestamp = self.current_timestamp();
        let seq;

        if timestamp == self.last_timestamp {
            seq = self.sequence + 1;

            if seq >= 1 << self.sequence_bits {
                std::thread::sleep(Duration::from_micros(1));
                return self.next_id();
            }
        } else {
            seq = 0;
            self.last_timestamp = timestamp;
        }

        self.sequence = seq;

        // Combine the timestamp, node_id, machine_id (if provided), and sequence to generate a unique ID
        let id = ((timestamp as u64)
            << (self.node_id_bits + self.machine_id_bits + self.sequence_bits))
            | ((self.node_id as u64) << (self.machine_id_bits + self.sequence_bits))
            | ((self.machine_id.unwrap_or(0) as u64) << self.sequence_bits)
            | seq as u64;

        Ok(id)
    }

    /// Get the current timestamp based on the epoch.
    ///
    /// # Returns
    ///
    /// An i64 timestamp.
    #[inline(always)]
    fn current_timestamp(&self) -> i64 {
        self.epoch.elapsed().as_millis() as i64
    }
}

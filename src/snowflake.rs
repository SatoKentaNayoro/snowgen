use std::sync::atomic::{AtomicI64, AtomicU16, Ordering};
use std::time::{Duration, Instant};

pub struct Snowflake {
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

impl Snowflake {
    // Generate the next unique ID
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
        let id = ((timestamp as u64) << (self.node_id_bits + self.machine_id_bits + self.sequence_bits))
            | ((self.node_id as u64) << (self.machine_id_bits + self.sequence_bits))
            | ((self.machine_id.unwrap_or(0) as u64) << self.sequence_bits)
            | seq as u64;

        Ok(id)
    }

    #[inline(always)]
    fn current_timestamp(&self) -> i64 {
        self.epoch.elapsed().as_millis() as i64
    }
}

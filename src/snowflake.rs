use std::sync::atomic::{AtomicI64, AtomicU16, Ordering};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

pub struct Snowflake {
    pub(crate) node_id: i32,
    pub(crate) machine_id: Option<i32>,
    pub(crate) epoch: Instant,
    pub(crate) timestamp_bits: u8,
    pub(crate) node_id_bits: u8,
    pub(crate) machine_id_bits: u8,
    pub(crate) sequence_bits: u8,
    pub(crate) last_timestamp: AtomicI64,
    pub(crate) sequence: AtomicU16,
}

impl Snowflake {
    // Generate the next unique ID
    pub fn next_id(&self) -> Result<u64, &'static str> {
        let mut last_timestamp = self.last_timestamp.load(Ordering::Relaxed);
        let mut sequence = self.sequence.load(Ordering::Relaxed);
        let mut timestamp = (Instant::now() - self.epoch).as_millis() as i64;

        // Ensure the clock is moving forward
        if timestamp < last_timestamp {
            return Err("Clock moved backwards. Refusing to generate ID.");
        }

        // If the timestamp is the same as the last generated ID, we need to increase the sequence number
        if timestamp == last_timestamp {
            sequence = (sequence + 1) & ((1 << self.sequence_bits) - 1);

            // Sequence overflow, wait for the next millisecond
            if sequence == 0 {
                while timestamp <= last_timestamp {
                    timestamp = (Instant::now() - self.epoch).as_millis() as i64;
                }
            }
        } else {
            sequence = 0;
        }

        self.last_timestamp.store(timestamp, Ordering::Relaxed);
        self.sequence.store(sequence, Ordering::Relaxed);

        // Combine the timestamp, node_id, machine_id (if provided), and sequence to generate a unique ID
        let id = ((timestamp as u64) << (self.node_id_bits + self.machine_id_bits + self.sequence_bits))
            | ((self.node_id as u64) << (self.machine_id_bits + self.sequence_bits))
            | ((self.machine_id.unwrap_or(0) as u64) << self.sequence_bits)
            | sequence as u64;

        Ok(id)
    }

    // Get the current time in milliseconds since UNIX epoch
    #[inline(always)]
    fn current_time() -> Result<i64, &'static str> {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_millis() as i64)
            .map_err(|_| "Failed to get current time.")
    }
}

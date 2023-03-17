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
    pub(crate) last_timestamp: AtomicI64,
    pub(crate) sequence: AtomicU16,
}

impl Snowflake {
    // Generate the next unique ID
    pub fn next_id(&self) -> Result<u64, &'static str> {
        let mut timestamp;
        let mut seq;

        loop {
            timestamp = self.current_timestamp();
            let last_timestamp = self.last_timestamp.load(Ordering::SeqCst);
            if timestamp == last_timestamp {
                let current_seq = self.sequence.load(Ordering::SeqCst);
                seq = current_seq + 1;

                if seq >= 1 << self.sequence_bits {
                    std::thread::sleep(Duration::from_micros(1));
                    continue;
                }

                match self.sequence.compare_exchange(current_seq, seq, Ordering::SeqCst, Ordering::Relaxed) {
                    Ok(_) => break,
                    Err(_) => {
                        std::thread::yield_now();
                        continue;
                    }
                }
            } else {
                seq = 0;
                match self.last_timestamp.compare_exchange(last_timestamp, timestamp, Ordering::SeqCst, Ordering::Relaxed) {
                    Ok(_) => {
                        self.sequence.store(seq, Ordering::SeqCst);
                        break;
                    }
                    Err(_) => {
                        std::thread::yield_now();
                        continue;
                    }
                }
            }
        }

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

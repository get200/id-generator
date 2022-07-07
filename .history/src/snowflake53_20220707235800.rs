use chrono::{TimeZone, Utc};
use core::panic;
use lazy_static::lazy_static;
use log::error;
use std::sync::{Arc, Mutex};
// use std::{thread, time::Duration};

#[derive(Debug, Default)]
struct Snowflake {
    sequence: i64,
    last_timestamp: i64,
    config: SnowflakeConfig,
}

impl Snowflake {
    pub fn new() -> Self {
        Snowflake::default()
    }

    pub fn set_config(&mut self, config: SnowflakeConfig) {
        if config.is_valid() {
            self.config = config;
        } else {
            panic!("invalid snowflake config!")
        }
    }

    fn get_timestamp(&self) -> i64 {
        Utc::now().timestamp() - self.config.base_seconds
    }

    fn next_timestamp(&self, timestamp: i64) -> i64 {
        loop {
            let next_timestamp = self.get_timestamp();
            if next_timestamp > timestamp {
                return next_timestamp;
            }
        }
    }

    fn next_id(&mut self) -> i64 {
        let mut current_timestamp = self.get_timestamp();
        let mut is_back = false;

        if current_timestamp < self.last_timestamp {
            error!(
                "clock is moving backwards. [{}]",
                current_timestamp - self.last_timestamp
            );
            is_back = true;
            current_timestamp = self.last_timestamp;
        }

        if current_timestamp == self.last_timestamp {
            self.sequence = (self.sequence + 1) & self.config.sequence_mask();
            if self.sequence == 0 {
                if is_back {
                    current_timestamp += 1;
                } else {
                    current_timestamp = self.next_timestamp(current_timestamp);
                }
            }
        } else {
            self.sequence = 0;
        };

        self.last_timestamp = current_timestamp;

        current_timestamp << self.config.timestamp_shift()
            | self.config.worker_id << self.config.worker_id_shift()
            | self.sequence
    }
}

#[derive(Debug)]
pub struct SnowflakeConfig {
    pub base_seconds: i64,
    pub worker_id: i64,
    worker_id_bits: u8,
    sequence_bits: u8,
}

impl Default for SnowflakeConfig {
    fn default() -> Self {
        Self {
            base_seconds: Utc.ymd(2022, 1, 1).and_hms(0, 0, 0).timestamp(),
            worker_id: 0,
            worker_id_bits: 5,
            sequence_bits: 16,
        }
    }
}

impl SnowflakeConfig {
    pub fn new() -> Self {
        Self::default()
    }

    fn is_valid(&self) -> bool {
        let mask = -1_i64 << self.worker_id_bits;
        if self.worker_id & mask != 0 {
            error!("invalid worker_id {}.", self.worker_id);
            return false;
        }
        true
    }

    #[inline(always)]
    fn sequence_mask(&self) -> i64 {
        !(-1_i64 << self.sequence_bits)
    }

    #[inline(always)]
    fn timestamp_shift(&self) -> u8 {
        self.worker_id_bits + self.sequence_bits
    }

    #[inline(always)]
    fn worker_id_shift(&self) -> u8 {
        self.sequence_bits
    }
}

lazy_static! {
    static ref SNOWFLAKE53: Arc<Mutex<Snowflake>> = Arc::new(Mutex::new(Snowflake::new()));
}

pub fn set_config(config: SnowflakeConfig) {
    SNOWFLAKE53.lock().unwrap().set_config(config)
}

pub fn next_id() -> i64 {
    SNOWFLAKE5..lock().unwrap().next_id()
}

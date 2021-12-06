use chrono::{TimeZone, Utc};
use core::panic;
use lazy_static::lazy_static;
use log::error;
use std::sync::{Arc, Mutex};
// use std::{thread, time::Duration};

#[derive(Debug, Default)]
struct Snowflake {
    epoch: i64,
    datacenter_id: i64,
    worker_id: i64,
    sequence: i64,
    last_timestamp: i64,

    datacenter_id_bits: u8,
    worker_id_bits: u8,
    sequence_bits: u8,

    timestamp_left_shift: u8,
    datacenter_id_left_shift: u8,
    worker_id_left_shift: u8,
}

impl Snowflake {
    pub fn new(datacenter_id: i64, worker_id: i64) -> Self {
        let mut snowflake = Snowflake::default();
        snowflake.epoch = Utc.ymd(2021, 1, 1).and_hms(0, 0, 0).timestamp_millis();
        snowflake.init(5, 5, 12);
        snowflake.set_datacenter_id(datacenter_id);
        snowflake.set_worker_id(worker_id);
        snowflake
    }

    fn set_datacenter_id(&mut self, datacenter_id: i64) {
        let mask = -1_i64 << self.datacenter_id_bits;
        if datacenter_id & mask != 0 {
            panic!("invalid datacenter_id {}.", datacenter_id);
        }
        self.datacenter_id = datacenter_id;
    }

    fn set_worker_id(&mut self, worker_id: i64) {
        let mask = -1_i64 << self.worker_id_bits;
        if worker_id & mask != 0 {
            panic!("invalid worker_id {}.", worker_id);
        }
        self.worker_id = worker_id;
    }

    fn set_config(&mut self, datacenter_id: i64, worker_id: i64) {
        self.set_datacenter_id(datacenter_id);
        self.set_worker_id(worker_id);
    }

    fn init(&mut self, datacenter_id_bits: u8, worker_id_bits: u8, sequence_bits: u8) {
        self.datacenter_id_bits = datacenter_id_bits;
        self.worker_id_bits = worker_id_bits;
        self.sequence_bits = sequence_bits;

        self.timestamp_left_shift = self.datacenter_id_bits + self.worker_id_bits + self.sequence_bits;
        self.datacenter_id_left_shift = self.worker_id_bits + self.sequence_bits;
        self.worker_id_left_shift = self.sequence_bits;
    }

    fn get_timestamp(&self) -> i64 {
        Utc::now().timestamp_millis() - self.epoch
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
            let mask = !(-1_i64 << self.sequence_bits);
            self.sequence = (self.sequence + 1) & mask;
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

        current_timestamp << self.timestamp_left_shift
            | self.datacenter_id << self.datacenter_id_left_shift
            | self.worker_id << self.worker_id_left_shift
            | self.sequence
    }
}

lazy_static! {
    static ref SNOWFLAKE: Arc<Mutex<Snowflake>> = Arc::new(Mutex::new(Snowflake::new(0, 0)));
}

pub fn set_config(datacenter_id: i64, worker_id: i64) {
    SNOWFLAKE.lock().unwrap().set_config(datacenter_id, worker_id)
}

pub fn next_id() -> i64 {
    SNOWFLAKE.lock().unwrap().next_id()
}

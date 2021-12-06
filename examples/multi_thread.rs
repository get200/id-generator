use chrono::Utc;
use id_generator::snowflake;

use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let start = Utc::now().timestamp_millis();
    let max_count = 50_000;
    let count = Arc::new(AtomicI64::new(0));

    let mut handles = Vec::new();
    for _ in 0..12 {
        let count = count.clone();
        handles.push(thread::spawn(move || loop {
            let _id = snowflake::next_id();
            let count = count.fetch_add(1, Ordering::Relaxed);
            if count >= max_count {
                return;
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let end = Utc::now().timestamp_millis();
    println!("Done: takes {} ms.", end - start);
}

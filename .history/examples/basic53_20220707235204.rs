use chrono::{TimeZone, Utc};
use id_generator::snowflake53::{self, SnowflakeConfig};

fn main() {
    let mut config = SnowflakeConfig::new();
    config.base_seconds = Utc.ymd(2021, 1, 1).and_hms(0, 0, 0).timestamp();
    config.worker_id = 0;
    // ...
    snowflake53::set_config(config);

    let id = snowflake53::next_id();
    println!("{}", id);
}

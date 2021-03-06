## Example

Dependencies:

```toml
[dependencies]
id-generator = "0.3.0"
```

Code:

```rust
/// basic
use chrono::{TimeZone, Utc};
use id_generator::snowflake::{self, SnowflakeConfig};

fn main() {
    let mut config = SnowflakeConfig::new();
    config.base_timestamp = Utc.ymd(2022, 1, 1).and_hms(0, 0, 0).timestamp_millis();
    config.datacenter_id = 0;
    config.worker_id = 0;
    // ...
    snowflake::set_config(config);

    let id = snowflake::next_id();
    println!("{}", id);
}

/// basic53
use chrono::{TimeZone, Utc};
use id_generator::snowflake53::{self, SnowflakeConfig};

fn main() {
    let mut config = SnowflakeConfig::new();
    config.base_seconds = Utc.ymd(2022, 1, 1).and_hms(0, 0, 0).timestamp();
    config.worker_id = 0;
    // ...
    snowflake53::set_config(config);

    let id = snowflake53::next_id();
    println!("{}", id);
}

```
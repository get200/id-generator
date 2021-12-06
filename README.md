## Example

Dependencies:

```toml
[dependencies]
id-generator = "0.1.0"
```

Code:

```rust
use id_generator::snowflake;

fn main() {
    snowflake::set_config(0, 0);
    let id = snowflake::next_id();
    println!("{}", id);
}

```
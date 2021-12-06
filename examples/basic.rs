use id_generator::snowflake;

fn main() {
    snowflake::set_config(0, 0);
    let id = snowflake::next_id();
    println!("{}", id);
}
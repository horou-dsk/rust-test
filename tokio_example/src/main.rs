use std::time::Instant;
use tokio::time::Duration;

#[cfg(debug_assertions)]
const DEV: bool = true;

#[cfg(not(debug_assertions))]
const DEV: bool = false;

fn main() {
    if DEV {
        let now = Instant::now() + Duration::from_nanos(100);
        println!("{}", Instant::now() > now);
        println!("开发");
    } else {
        let now = Instant::now() + Duration::from_nanos(100);
        println!("{}", Instant::now() > now);
        println!("生产");
    }
}
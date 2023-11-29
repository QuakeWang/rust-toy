use std::time::Duration;

use tokio::time;

#[tokio::main]
async fn main() {
    // Create a instance of interval timer
    let mut interval = time::interval(Duration::from_millis(10));
    // Execute the interval timer
    interval.tick().await;
    // After 10ms
    interval.tick().await;
    // After 20ms
    interval.tick().await;
}

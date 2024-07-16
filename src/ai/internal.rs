use std::io;
use std::io::Write;

use tokio::time::{interval, Duration, Interval};

pub async fn wait_for_response(mut rx_stop: tokio::sync::mpsc::Receiver<()>) {
    let mut interval: Interval = interval(Duration::from_millis(100));
    let mut idx = 0;
    let spinner = ["-", "\\", "|", "/"];

    loop {
        tokio::select! {
            _ = interval.tick() =>{
                print!("Waiting for response...{}",spinner[idx]);
                io::stdout().flush().unwrap();
                idx = (idx + 1) %spinner.len();
                print!("\r");
            },
            result = rx_stop.recv() =>{
                match result{
                    Some(_) | None => break,
                }
            }
        }
    }
}

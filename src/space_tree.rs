use log::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tokio::time::Duration;

use space_time_trees::*;

pub static SPACE_TREE_BUFFER_MAINTAIN_RATE: u64 = 1; // milliseconds

#[tokio::main]
async fn main() -> () {
    let buffer = Arc::new(Mutex::new(HashMap::<String, TransformStamped>::new()));
    let buffer_clone = buffer.clone();
    tokio::task::spawn(async move {
        match maintain_space_tree_buffer(&buffer_clone, SPACE_TREE_BUFFER_MAINTAIN_RATE).await {
            Ok(()) => (),
            Err(e) => error!("Space tree buffer maintainer failed with: '{}'.", e),
        };
    });

    let handle = std::thread::spawn(move || loop {
        std::thread::sleep(Duration::from_millis(1000));
    });

    handle.join().unwrap();

}

use log::*;
use nalgebra::Isometry3;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tokio::time::{Duration, Instant};

use space_time_trees::*;

pub static SPACE_TREE_BUFFER_MAINTAIN_RATE: u64 = 100; // milliseconds

#[tokio::main]
async fn main() -> () {
    let asdf = HashMap::from([("test_transform".to_string(), TransformStamped {
        time_stamp: Instant::now(),
        child_frame_id: "child".to_string(),
        parent_frame_id: "parent".to_string(),
        transform: Isometry3::default(),
        json_metadata: "{asdf: asdf}".to_string()
    })]);
    // let buffer = Arc::new(Mutex::new(HashMap::<String, TransformStamped>::new()));
    let buffer = Arc::new(Mutex::new(asdf));
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

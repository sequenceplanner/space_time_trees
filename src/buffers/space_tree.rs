use crate::TransformStamped;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::time::{Duration, Instant};

pub async fn maintain_space_tree_buffer(
    buffer: &Arc<Mutex<HashMap<String, TransformStamped>>>,
    maintain_rate: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let buffer_local = buffer.lock().unwrap().clone();
        let mut updated_buffer = HashMap::new();

        buffer_local.iter().for_each(|(name, frame)| {
            let _insert_res = updated_buffer.insert(name.to_string(), TransformStamped {
                time_stamp: Instant::now(),
                parent_frame_id: frame.parent_frame_id.clone(),
                child_frame_id: frame.child_frame_id.clone(),
                transform: frame.transform,
                json_metadata: frame.json_metadata.clone()
            });
        });

        for b in &buffer_local {
            println!("{:#?}", b)
        }

        *buffer.lock().unwrap() = updated_buffer;

        std::thread::sleep(Duration::from_millis(maintain_rate));
    }
}

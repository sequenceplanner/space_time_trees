use log::{info, warn, error};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::*;

pub async fn add_frames(
    buffer: &Arc<Mutex<HashMap<String, TransformStamped>>>,
    frames: &Vec<TransformStamped>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer_local = buffer.lock().unwrap().clone();
    // let mut updated_buffer = HashMap::new();
    for frame in frames {
        if frame.child_frame_id == "world" {
            error!("Frame name 'world' is reserved.")
        } else {
            match buffer_local.insert(frame.child_frame_id.clone(), frame.clone()) {
                Some(_) => warn!("Frame '{}' already exists, now updated", frame.child_frame_id),
                None => info!("Frame '{}' added as child of frame '{}'", frame.child_frame_id, frame.parent_frame_id)
            }
        }
    }
    *buffer.lock().unwrap() = buffer_local;
    Ok(())
}
use serde_json::Value;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::BufReader,
};
use tokio::time::Instant;

use crate::*;

pub fn list_frames_in_dir(path: &str) -> Result<Vec<String>, Box<dyn std::error::Error + Send>> {
    let mut scenario = vec![];
    match fs::read_dir(path) {
        Ok(dir) => dir.for_each(|file| match file {
            Ok(entry) => match entry.path().to_str() {
                Some(valid) => scenario.push(valid.to_string()),
                None => {
                    log::warn!(target: "space_time_trees", "Scenario path is not valid unicode.")
                }
            },
            Err(e) => log::warn!(target: "space_time_trees", "Reading entry failed with '{}'.", e),
        }),
        Err(e) => {
            log::warn!(target: "space_time_trees",
                "Reading the scenario directory failed with: '{}'.",
                e
            );
            log::warn!(target: "space_time_trees", "Empty scenario is loaded.");
            return Err(Box::new(ErrorMsg::new(&format!(
                "Reading the scenario directory failed with: '{}'. 
                    Empty scenario is loaded.",
                e
            ))));
        }
    }
    Ok(scenario)
}

pub fn load_new_scenario(scenario: &Vec<String>) -> HashMap<String, TransformStamped> {
    let mut transforms_stamped = HashMap::new();

    for path in scenario {
        let json = match load_json_from_file(path) {
            Some(json) => json,
            None => continue,
        };

        let child_frame_id = match extract_string_field(&json, "child_frame_id") {
            Some(id) => id,
            None => continue,
        };

        let parent_frame_id = match extract_string_field(&json, "parent_frame_id") {
            Some(id) => id,
            None => continue,
        };

        let transform = match extract_transform(&json) {
            Some(transform) => transform,
            None => continue,
        };

        let json_metadata = match extract_string_field(&json, "json_metadata") {
            Some(metadata) => metadata,
            None => "".to_string(),
        };

        transforms_stamped.insert(
            child_frame_id.clone(),
            TransformStamped {
                time_stamp: Instant::now(),
                child_frame_id,
                parent_frame_id,
                transform: json_transform_to_isometry(transform),
                json_metadata,
            },
        );
    }

    transforms_stamped
}

fn load_json_from_file(path: &str) -> Option<Value> {
    match File::open(path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            match serde_json::from_reader(reader) {
                Ok(json) => Some(json),
                Err(e) => {
                    log::warn!(target: "space_time_trees", 
                        concat!(
                            "Deserialization failed with: '{}'. ",
                            "The JSON file may be malformed or contain ",
                            "unexpected data."
                        ),
                        e
                    );
                    None
                }
            }
        },
        Err(e) => {
            log::warn!(target: "space_time_trees", 
                concat!(
                    "Opening json file failed with: '{}'. ",
                    "Please check if the file path is correct and ",
                    "you have sufficient permissions."
                ),
                e
            );
            None
        }
    }
}

fn extract_string_field(json: &Value, field: &str) -> Option<String> {
    match json.get(field).and_then(|v| v.as_str()) {
        Some(value) => Some(value.to_string()),
        None => {
            log::warn!(target: "space_time_trees", 
                concat!(
                    "Invalid or missing '{}'. ",
                    "Ensure the '{}' field is present and ",
                    "is a valid string."
                ),
                field, field
            );
            None
        }
    }
}

fn extract_transform(json: &Value) -> Option<JsonTransform> {
    match json.get("transform") {
        Some(value) => match serde_json::from_value(value.clone()) {
            Ok(transform) => Some(transform),
            Err(e) => {
                log::warn!(target: "space_time_trees", 
                    concat!(
                        "Failed to deserialize 'transform' field: '{}'. ",
                        "Ensure the 'transform' field is correctly formatted."
                    ),
                    e
                );
                None
            }
        },
        None => {
            log::warn!(target: "space_time_trees", 
                concat!(
                    "Missing 'transform' field. ",
                    "Ensure the 'transform' field is present in the JSON."
                )
            );
            None
        }
    }
}

#[test]
fn test_load_and_deserialize_from_file() {
    fn initialize_logging() {
        std::env::set_var("RUST_LOG", "warn");
        let _ = env_logger::builder().is_test(true).try_init();
    }

    initialize_logging();

    log::warn!("Starting the test_deserialize_transform_stamped test...");

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not set");

    let path = format!("{}/tests/data", manifest_dir);
    println!("{}", path);
    let frames = list_frames_in_dir(&path);

    match frames {
        Ok(frames) => {
            // println!("Frames: {:?}", frames);
            let scenario = load_new_scenario(&frames);
            println!("{:#?}", scenario);
        }
        _ => panic!(),
    }
}

// pub fn load_overlay_scenario

// pub async fn reload_scenario(
//     message: &r2r::scene_manipulation_msgs::srv::ManipulateExtras::Request,
//     broadcasted_frames: &Arc<Mutex<HashMap<String, FrameData>>>,
//     node_id: &str,
// ) -> ManipulateExtras::Response {
//     match list_frames_in_dir(&message.scenario_path, node_id).await {
//         Ok(scenario) => {
//             let loaded = load_scenario(&scenario, node_id);
//             let mut local_broadcasted_frames = broadcasted_frames.lock().unwrap().clone();
//             for x in &loaded {
//                 local_broadcasted_frames.insert(x.0.clone(), x.1.clone());
//             }
//             *broadcasted_frames.lock().unwrap() = local_broadcasted_frames;
//             extra_success_response(&format!(
//                 "Reloaded frames in the scene: '{:?}'.",
//                 loaded.keys()
//             ))
//         }
//         Err(e) => extra_error_response(&format!("Reloading the scenario failed with: '{:?}'.", e)),
//     }
// }

// async fn persist_frame_change(path: &str, frame: FrameData) -> bool {
//     match fs::read_dir(path) {
//         Ok(dir) => dir.for_each(|file| match file {
//             Ok(entry) => match entry.path().to_str() {
//                 Some(valid) => match valid.to_string() == format!("{}{}", path, frame.child_frame_id.clone()) {
//                     true => {
//                         println!("Changing existing frame {} permanently", frame.child_frame_id.clone());
//                         match File::open(valid.clone()) {
//                             Ok(file) =>
//                         }
//                         let writer = BufWriter::;
//                     // }
//                     },
//                     false => {}
//                 }
//                 None => r2r::log_warn!(NODE_ID, "Path is not valid unicode."),
//             },
//             Err(e) => r2r::log_warn!(NODE_ID, "Reading entry failed with '{}'.", e),
//         }),
//         Err(e) => {
//             r2r::log_warn!(
//                 NODE_ID,
//                 "Reading the scenario directory failed with: '{}'.",
//                 e
//             );
//             r2r::log_warn!(NODE_ID, "Empty scenario is loaded/reloaded.");
//             return false
//         }
//     }
//     true
// }

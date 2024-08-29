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
    let mut transforms_stamped: HashMap<String, TransformStamped> = HashMap::new();

    for path in scenario {
        match File::open(path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                match serde_json::from_reader(reader) {
                    Ok::<Value, _>(json) => {
                        let child_frame_id = match json.get("child_frame_id") {
                            Some(value) => match value.as_str() {
                                Some(child_frame_id) => child_frame_id,
                                None => {
                                    log::warn!(target: "space_time_trees", "Field 'child_frame_id' should be a string.");
                                    continue;
                                }
                            },
                            None => {
                                log::warn!(target: "space_time_trees", "Json file doens't contain field 'child_frame_id'.");
                                continue;
                            }
                        };
                        let parent_frame_id = match json.get("parent_frame_id") {
                            Some(value) => match value.as_str() {
                                Some(parent_frame_id) => parent_frame_id,
                                None => {
                                    log::warn!(target: "space_time_trees", "Field 'parent_frame_id' should be a string.");
                                    continue;
                                }
                            },
                            None => {
                                log::warn!(target: "space_time_trees", "Json file doens't contain field 'parent_frame_id'.");
                                continue;
                            }
                        };
                        let transform: JsonTransform = match json.get("transform") {
                            Some(value) => match serde::Deserialize::deserialize(value) {
                                Ok(transform) => transform,
                                Err(e) => {
                                    log::warn!(target: "space_time_trees", "Deserializing the field 'transform' filed with: {}.", e);
                                    continue;
                                }
                            },
                            None => {
                                log::warn!(target: "space_time_trees", "Json file doens't contain field 'transform'.");
                                continue;
                            }
                        };
                        let json_metadata = match json.get("json_metadata") {
                            Some(value) => match value.as_str() {
                                Some(json_metadata) => json_metadata.to_string(), // unstructured for now
                                None => {
                                    log::warn!(target: "space_time_trees", "No metadata detected for frame: {}.'.", child_frame_id);
                                    log::warn!(target: "space_time_trees", "Empty metadata loaded for frame: {}.'.", child_frame_id);
                                    "".to_string()
                                }
                            },
                            None => "".to_string(),
                        };

                        transforms_stamped.insert(
                            child_frame_id.to_string(),
                            TransformStamped {
                                time_stamp: Instant::now(),
                                child_frame_id: child_frame_id.to_string(),
                                parent_frame_id: parent_frame_id.to_string(),
                                transform: json_transform_to_isometry(transform),
                                json_metadata,
                            },
                        );
                    }
                    Err(e) => {
                        log::warn!(target: "space_time_trees", "Deserialization failed with: '{}'.", e)
                    }
                }
            }
            Err(e) => {
                log::warn!(target: "space_time_trees", "Opening json file failed with: '{}'.", e)
            }
        }
    }

    transforms_stamped
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
            println!("Frames: {:?}", frames);
            let scenario = load_new_scenario(&frames);
            println!("{:?}", scenario);
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

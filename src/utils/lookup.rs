use crate::TransformStamped;
use nalgebra::{Isometry, Isometry3};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::current;
use tokio::time::{interval, Duration, Instant, Interval};

pub static MAX_TRANSFORM_CHAIN: u64 = 1000;

pub fn isometry_chain_product(vec: Vec<Isometry3<f64>>) -> Isometry3<f64> {
    vec.iter().fold(Isometry3::identity(), |a, &b| a * b)
}

// pub fn lookup_transform(
//     root_frame_id: &str,
//     parent_frame_id: &str,
//     child_frame_id: &str,
//     buffer: &Arc<Mutex<HashMap<String, TransformStamped>>>,
// ) -> Option<TransformStamped> {
//     let buffer_local = buffer.lock().unwrap().clone();
//     let mut chain = vec![];
//     match is_cyclic_all(&frames_local) {
//         (false, _) => match parent_to_world(parent_frame_id, &frames_local) {
//             Some(up_chain) => match world_to_child(child_frame_id, &frames_local) {
//                 Some(down_chain) => {
//                     chain.extend(up_chain);
//                     chain.extend(down_chain);
//                     Some(affine_to_tf(&chain.iter().product::<DAffine3>()))
//                 }
//                 None => None,
//             },
//             None => None,
//         },
//         (true, _cause) => None,
//     }
// }

pub fn parent_to_root(
    parent_frame_id: &str,
    root_frame_id: &str,
    buffer: &HashMap<String, TransformStamped>,
) -> Option<Isometry3<f64>> {
    let mut current_parent = parent_frame_id.to_string();
    let mut path = vec![];
    let mut length = 0;
    let res = loop {
        if length >= MAX_TRANSFORM_CHAIN {
            break None;
        } else {
            length = length + 1;
            match buffer.get(&current_parent) {
                Some(parent) => {
                    path.push(parent.transform.inverse());
                    if parent.parent_frame_id == root_frame_id {
                        break Some(path);
                    } else {
                        current_parent = parent.parent_frame_id.to_string();
                        // continue;
                    }
                }
                None => break None,
            }
        }
    };

    match res {
        Some(chain) => Some(isometry_chain_product(chain)),
        None => None,
    }
}

#[cfg(test)]
mod tests {

    use nalgebra::{Isometry3, Quaternion, Translation, UnitQuaternion, Vector3};
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use tokio::time::{interval, timeout, Duration, Instant};

    use crate::*;
    use log::*;

    #[test]
    fn test_parent_to_root() {
        let test_buffer = HashMap::from([
            (
                "finger".to_string(),
                TransformStamped {
                    time_stamp: Instant::now(),
                    child_frame_id: "finger".to_string(),
                    parent_frame_id: "hand".to_string(),
                    transform: Isometry3 {
                        translation: Translation {
                            vector: Vector3::new(0.0, 0.0, 0.0),
                        },
                        rotation: UnitQuaternion::from_quaternion(Quaternion::new(
                            1.0, 0.0, 0.0, 0.0,
                        )),
                    },
                    json_metadata: "{foo: bar}".to_string(),
                },
            ),
            (
                "hand".to_string(),
                TransformStamped {
                    time_stamp: Instant::now(),
                    child_frame_id: "hand".to_string(),
                    parent_frame_id: "elbow".to_string(),
                    transform: Isometry3 {
                        translation: Translation {
                            vector: Vector3::new(1.0, 0.0, 0.0),
                        },
                        rotation: UnitQuaternion::from_quaternion(Quaternion::new(
                            0.7071, 0.7071, 0.0, 0.0,
                        )),
                    },
                    json_metadata: "{foo: bar}".to_string(),
                },
            ),
            (
                "elbow".to_string(),
                TransformStamped {
                    time_stamp: Instant::now(),
                    child_frame_id: "elbow".to_string(),
                    parent_frame_id: "shoulder".to_string(),
                    transform: Isometry3 {
                        translation: Translation {
                            vector: Vector3::new(0.0, 1.0, 0.0),
                        },
                        rotation: UnitQuaternion::from_quaternion(Quaternion::new(
                            0.7071, 0.0, 0.7071, 0.0,
                        )),
                    },
                    json_metadata: "{foo: bar}".to_string(),
                },
            ),
            (
                "shoulder".to_string(),
                TransformStamped {
                    time_stamp: Instant::now(),
                    child_frame_id: "shoulder".to_string(),
                    parent_frame_id: "world".to_string(),
                    transform: Isometry3 {
                        translation: Translation {
                            vector: Vector3::new(0.0, 0.0, 1.0),
                        },
                        rotation: UnitQuaternion::from_quaternion(Quaternion::new(
                            0.7071, 0.0, 0.0, 0.7071,
                        )),
                    },
                    json_metadata: "{foo: bar}".to_string(),
                },
            ),
        ]);

        let res = parent_to_root("hand", "world", &test_buffer);
        assert!(!res.is_none());
        println!("{}", res.unwrap());
        // TODO: verify if this is correct and test
    }
}

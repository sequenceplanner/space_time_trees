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

// Search the tree upstream to the root
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

// BFS to get the path to the child...
pub fn root_to_child(
    child_frame_id: &str,
    root_frame_id: &str,
    buffer: &HashMap<String, TransformStamped>,
) -> Option<Isometry3<f64>> {
    let mut length = 0;
    let mut stack = vec![];
    get_frame_children(root_frame_id, buffer)
        .iter()
        .for_each(|(k, v)| stack.push((k.to_string(), vec![k.to_string()], vec![v.transform])));

    let res = loop {
        if length >= MAX_TRANSFORM_CHAIN {
            break None;
        } else {
            length = length + 1;
            match stack.pop() {
                Some((frame, path, chain)) => {
                    if frame == child_frame_id {
                        break Some(chain);
                    } else {
                        get_frame_children(&frame, buffer)
                            .iter()
                            .for_each(|(k, v)| {
                                let mut prev_path = path.clone();
                                let mut prev_chain = chain.clone();
                                prev_path.push(k.clone());
                                prev_chain.push(v.transform);
                                stack.insert(
                                    0,
                                    (k.to_string(), prev_path.clone(), prev_chain.clone()),
                                )
                            })
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

// The frame whose children we are searching for don't have to exist in the transform buffer
pub fn get_frame_children(
    frame: &str,
    buffer: &HashMap<String, TransformStamped>,
) -> Vec<(String, TransformStamped)> {
    buffer
        .iter()
        .filter(|(_, v)| v.parent_frame_id == frame)
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect()
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

    fn dummy_1_frame() -> TransformStamped {
        TransformStamped {
            time_stamp: Instant::now(),
            parent_frame_id: "world".to_string(),
            child_frame_id: "dummy_1".to_string(),
            transform: Isometry3::default(),
            json_metadata: String::default(),
        }
    }

    fn dummy_2_frame() -> TransformStamped {
        TransformStamped {
            time_stamp: Instant::now(),
            parent_frame_id: "dummy_1".to_string(),
            child_frame_id: "dummy_2".to_string(),
            transform: Isometry3::default(),
            json_metadata: String::default(),
        }
    }

    fn dummy_3_frame() -> TransformStamped {
        TransformStamped {
            time_stamp: Instant::now(),
            parent_frame_id: "dummy_1".to_string(),
            child_frame_id: "dummy_3".to_string(),
            transform: Isometry3::default(),
            json_metadata: String::default(),
        }
    }

    #[test]
    fn test_get_frame_children() {
        let mut buffer = HashMap::<String, TransformStamped>::new();
        buffer.insert("dummy_1".to_string(), dummy_1_frame());

        //          w
        //          |
        //          d1

        assert_eq!(
            get_frame_children("world", &buffer)
                .iter()
                .map(|x| x.0.clone())
                .collect::<Vec<String>>(),
            vec!("dummy_1")
        );

        buffer.insert("dummy_2".to_string(), dummy_2_frame());

        //          w
        //          |
        //          d1
        //          |
        //          d2

        assert_eq!(
            get_frame_children("dummy_1", &buffer)
                .iter()
                .map(|x| x.0.clone())
                .collect::<Vec<String>>(),
            vec!("dummy_2")
        );

        assert_eq!(
            get_frame_children("world", &buffer)
                .iter()
                .map(|x| x.0.clone())
                .collect::<Vec<String>>(),
            vec!("dummy_1")
        );

        assert_eq!(
            get_frame_children("dummy_2", &buffer)
                .iter()
                .map(|x| x.0.clone())
                .collect::<Vec<String>>(),
            Vec::<String>::new()
        );

        buffer.insert("dummy_3".to_string(), dummy_3_frame());

        //          w
        //          |
        //          d1
        //         /  \
        //       d2    d3

        assert_eq!(
            get_frame_children("world", &buffer)
                .iter()
                .map(|x| x.0.clone())
                .collect::<Vec<String>>()
                .sort(),
            vec!("dummy_2", "dummy_3").sort()
        );
    }
}

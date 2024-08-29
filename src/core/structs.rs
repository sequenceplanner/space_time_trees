use nalgebra::{Isometry3, Quaternion, UnitQuaternion, Vector3};
use serde::Deserialize;
use structopt::StructOpt;
use tokio::time::Instant;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct JsonTranslation {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct JsonRotation {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct JsonTransform {
    pub translation: JsonTranslation,
    pub rotation: JsonRotation,
}

pub fn json_transform_to_isometry(json: JsonTransform) -> Isometry3<f64> {
    let translation = Vector3::new(json.translation.x, json.translation.y, json.translation.z);
    let rotation = UnitQuaternion::from_quaternion(Quaternion::new(
        json.rotation.w,
        json.rotation.x,
        json.rotation.y,
        json.rotation.z,
    ));

    Isometry3::from_parts(translation.into(), rotation)
}

// Isometry3 should be similar to the Transform message in ROS
#[derive(Debug, Clone, PartialEq)]
pub struct TransformStamped {
    // #[serde(deserialize_with = "deserialize_instant")]
    pub time_stamp: Instant,
    pub parent_frame_id: String,
    pub child_frame_id: String,
    // #[serde(deserialize_with = "deserialize_isometry_3_f64")]
    pub transform: Isometry3<f64>,
    pub json_metadata: String,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct ArgsCLI {
    /// Visualize Tree
    #[structopt(long, short = "v", parse(try_from_str), default_value = "false")]
    pub visualize: bool,
    // load and maintain,
    // load from file,
    // load from redis,
    // kill all,
    // etc...
}

pub struct Args {
    pub visualize: bool,
}

use tokio::time::Instant;
use nalgebra::Isometry3;

// Isometry3 is the same as Transform message in ROS
#[derive(Debug, Clone)]
pub struct TransformStamped {
    pub time_stamp: Instant,
    pub parent_frame_id: String,
    pub child_frame_id: String,
    pub transform: Isometry3<f64>,
    pub json_metadata: String
}
use tokio::time::Instant;
use nalgebra::Isometry3;
use structopt::StructOpt;

// Isometry3 is the same as Transform message in ROS
#[derive(Debug, Clone)]
pub struct TransformStamped {
    pub time_stamp: Instant,
    pub parent_frame_id: String,
    pub child_frame_id: String,
    pub transform: Isometry3<f64>,
    pub json_metadata: String
}

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct ArgsCLI {
    /// Visualize Tree
    #[structopt(long, short = "v", parse(try_from_str), default_value = "false")]
    pub visualize: bool,
}

pub struct Args {
    pub visualize: bool
}
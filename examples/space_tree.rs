use log::*;
use space_time_trees::utils::treeviz::vizualize_tree;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use space_time_trees::*;
use structopt::StructOpt;
use tokio::time::Duration;

pub static SPACE_TREE_BUFFER_MAINTAIN_RATE: u64 = 1; // milliseconds
pub static VISUALIZE_TREE_REFRESH_RATE: u64 = 100; // milliseconds

fn handle_args() -> Args {
    let args = ArgsCLI::from_args();
    Args {
        visualize: args.visualize,
    }
}

#[tokio::main]
async fn main() -> () {
    let args = handle_args();

    let buffer = Arc::new(Mutex::new(HashMap::<String, TransformStamped>::new()));
    let buffer_clone = buffer.clone();
    tokio::task::spawn(async move {
        match maintain_space_tree_buffer(&buffer_clone, SPACE_TREE_BUFFER_MAINTAIN_RATE).await {
            Ok(()) => (),
            Err(e) => error!("Space tree buffer maintainer failed with: '{}'.", e),
        };
    });

    let buffer_clone = buffer.clone();
    if args.visualize {
        tokio::task::spawn(async move {
            match vizualize_tree(&buffer_clone, VISUALIZE_TREE_REFRESH_RATE).await {
                Ok(()) => (),
                Err(e) => error!("Space tree buffer maintainer failed with: '{}'.", e),
            };
        });
    }

    let handle = std::thread::spawn(move || loop {
        std::thread::sleep(Duration::from_millis(1000));
    });

    handle.join().unwrap();
}

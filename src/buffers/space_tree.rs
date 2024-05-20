use crate::TransformStamped;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::time::{Duration, Instant};


// TODO: actually, this should just remove stale frames, the time stamps should be updated by the broadcasters
pub async fn maintain_space_tree_buffer(
    buffer: &Arc<Mutex<HashMap<String, TransformStamped>>>,
    maintain_rate: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let buffer_local = buffer.lock().unwrap().clone();
        let mut updated_buffer = HashMap::new();

        buffer_local.iter().for_each(|(name, frame)| {
            let _insert_res = updated_buffer.insert(
                name.to_string(),
                TransformStamped {
                    time_stamp: Instant::now(),
                    parent_frame_id: frame.parent_frame_id.clone(),
                    child_frame_id: frame.child_frame_id.clone(),
                    transform: frame.transform,
                    json_metadata: frame.json_metadata.clone(),
                },
            );
        });

        *buffer.lock().unwrap() = updated_buffer;
        tokio::time::sleep(Duration::from_millis(maintain_rate)).await;
    }
}

#[cfg(test)]
mod tests {

    use nalgebra::Isometry3;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use tokio::time::{interval, timeout, Duration, Instant};

    use crate::*;
    use log::*;

    #[tokio::test]
    async fn increment_interval() {
        let mut interval = interval(Duration::from_millis(10));
        let counter = Arc::new(Mutex::new(0));
        let counter_clone = counter.clone();
        let _ = timeout(Duration::from_millis(300), async move {
            loop {
                interval.tick().await;
                let counter_local = counter_clone.lock().unwrap().clone();
                *counter_clone.clone().lock().unwrap() = counter_local + 1;
            }
        })
        .await;
        let counter_assert = counter.lock().unwrap().clone();
        assert_eq!(counter_assert, 31)
    }

    // Pausing time has the effect that any time-related future may become ready early.
    // The condition for the time-related future resolving early is that there are no
    // more other futures which may become ready. This essentially fast-forwards time
    // when the only future being awaited is time-related.
    #[tokio::test(start_paused = true)]
    async fn increment_interval_with_paused_time() {
        let mut interval = interval(Duration::from_millis(10));
        let counter = Arc::new(Mutex::new(0));
        let counter_clone = counter.clone();
        let _ = timeout(Duration::from_millis(3000), async move {
            loop {
                interval.tick().await;
                let counter_local = counter_clone.lock().unwrap().clone();
                *counter_clone.clone().lock().unwrap() = counter_local + 1;
            }
        })
        .await;
        let counter_assert = counter.lock().unwrap().clone();
        assert_eq!(counter_assert, 301)
    }

    #[tokio::test(start_paused = true)]
    async fn space_tree_buffer_is_maintained() {
        let mut interval = interval(Duration::from_millis(50));

        let test_buffer = HashMap::from([
            (
                "test_transform".to_string(),
                TransformStamped {
                    time_stamp: Instant::now(),
                    child_frame_id: "child_1".to_string(),
                    parent_frame_id: "parent".to_string(),
                    transform: Isometry3::default(),
                    json_metadata: "{foo: bar}".to_string(),
                },
            ),
            (
                "test_transform_2".to_string(),
                TransformStamped {
                    time_stamp: Instant::now(),
                    child_frame_id: "child_2".to_string(),
                    parent_frame_id: "parent".to_string(),
                    transform: Isometry3::default(),
                    json_metadata: "{foo: bar}".to_string(),
                },
            ),
        ]);

        let mut acc_vec = vec![];
        acc_vec.push(test_buffer.clone());

        let buffer = Arc::new(Mutex::new(test_buffer));
        let buffer_clone = buffer.clone();

        let buffer_acc = Arc::new(Mutex::new(acc_vec));
        let buffer_acc_clone = buffer_acc.clone();

        let counter = Arc::new(Mutex::new(0));
        let counter_clone = counter.clone();

        tokio::task::spawn(async move {
            match maintain_space_tree_buffer(&buffer, 10).await {
                Ok(()) => (),
                Err(e) => error!("Space tree buffer maintainer failed with: '{}'.", e),
            };
        });

        let _ = timeout(Duration::from_millis(500), async move {
            loop {
                interval.tick().await;
                let counter_local = counter_clone.lock().unwrap().clone();
                *counter_clone.clone().lock().unwrap() = counter_local + 1;
                let mut buffer_acc_local = buffer_acc.lock().unwrap().clone();
                let buffer_clone_local = buffer_clone.lock().unwrap().clone();
                buffer_acc_local.push(buffer_clone_local);
                *buffer_acc.lock().unwrap() = buffer_acc_local;
            }
        })
        .await;
        let mut buffer_acc_clone_local = buffer_acc_clone.lock().unwrap().clone();
        buffer_acc_clone_local.drain(0..1);
        let truth_vec = buffer_acc_clone_local
            .windows(2)
            .map(|x| {
                let prev_test_transform = x[0].get("test_transform").unwrap();
                let next_test_transform = x[1].get("test_transform").unwrap();
                println!("{:?}", prev_test_transform.time_stamp);
                next_test_transform.time_stamp > prev_test_transform.time_stamp
            })
            .collect::<Vec<bool>>();
        assert!(truth_vec.iter().all(|f| *f));
    }
}

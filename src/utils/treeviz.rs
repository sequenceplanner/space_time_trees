use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::*;
use log::{info, warn};
use termtree::Tree;

pub static MAX_TRANSFORM_CHAIN: u64 = 1000;

pub fn get_tree_root(buffer: &HashMap<String, TransformStamped>) -> Option<String> {
    for frame in buffer {
        match buffer.get(&frame.0.clone()) {
            Some(_) => continue,
            None => return Some(frame.0.clone()),
        }
    }
    None
}



// pub struct Tree<D: Display> {
//     pub root: D,
//     pub leaves: Vec<Tree<D>>,
//     /* private fields */
// }


// use render_as_tree::Node;
// struct BasicNode {
//     pub name: String,
//     pub children: Vec<BasicNode>,
// }

// impl BasicNode {
//     pub fn new(name: String) -> BasicNode {
//         BasicNode {
//             name,
//             children: Vec::new(),
//         }
//     }
// }

// impl Node for BasicNode {
//     type Iter<'a> = std::slice::Iter<'a, Self>;

//     fn name(&self) -> &str {
//         &self.name
//     }
//     fn children(&self) -> Self::Iter<'_> {
//         self.children.iter()
//     }
// }


pub async fn vizualize_tree(
    buffer: &Arc<Mutex<HashMap<String, TransformStamped>>>,
    refresh_rate: u64,
) -> Result<(), Box<dyn std::error::Error>> {

    let mut length = 0;
    let mut stack = vec![];

    let buffer_local = buffer.lock().unwrap().clone();
        match get_tree_root(&buffer_local) {
            Some(root) => {

                fn evalasdf(root: String, buffer: HashMap<String, TransformStamped>, tree: Tree<String>) -> Tree<String> {
                    match get_frame_children(&root, &buffer) {
                        Some(children) => {
                            for child in children {
                                tree.push(evalasdf(child, buffer, tree));
                            } 
                        },
                        None => tree
                    }
                    // Tree::with_leaves(tree, get_frame_children(&root, &buffer)
                    // .iter().for_each(|x| evalasdf(x.0, buffer, tree))) // .collect::<Vec<String>>())
                    // Tree {
                    //     root: root.to_string(),
                    //     leaves: get_frame_children(&root, &buffer_local)
                    //     .iter().all(|x| eval(x, tree))
                    // }
                }

                // let mut tree = Tree {
                //     root: root.to_string(),
                //     leaves: get_frame_children(&root, &buffer_local)
                //     .iter()
                //     .map(|(k, v)| Tree {}k.clone()).collect::<Vec<String>>()
                // };


                loop {
                    if length >= MAX_TRANSFORM_CHAIN {
                        break None;
                    } else {
                        length = length + 1;
                        match stack.pop() {
                            Some(node) => {
                                let mut children = get_frame_children(node, &buffer_local)
                                .iter()
                                .for_each(|(k, v)| stack.push(k));
                            },
                            None => break None
                        }
                        None;
                    }

                
            },
            
        }
        None => println!("No tree to visualize."),
        
    }
    Ok(())
}

    

    // let res = loop {
    //     if length >= MAX_TRANSFORM_CHAIN {
    //         break None;
    //     } else {
    //         length = length + 1;
    //         match stack.pop() {
    //             Some((frame, path, chain)) => {
    //                 if frame == child_frame_id {
    //                     break Some(chain);
    //                 } else {
    //                     get_frame_children(&frame, buffer)
    //                         .iter()
    //                         .for_each(|(k, v)| {
    //                             let mut prev_path = path.clone();
    //                             let mut prev_chain = chain.clone();
    //                             prev_path.push(k.clone());
    //                             prev_chain.push(v.transform);
    //                             stack.insert(
    //                                 0,
    //                                 (k.to_string(), prev_path.clone(), prev_chain.clone()),
    //                             )
    //                         })
    //                 }
    //             }
    //             None => break None,
    //         }
    //     }
    // };

    // match res {
    //     Some(chain) => Some(isometry_chain_product(chain)),
    //     None => None,
    // }



    // loop {
    //     let buffer_local = buffer.lock().unwrap().clone();
    //     match get_tree_root(&buffer_local) {
    //         Some(root) => {
    //             let mut children = get_frame_children(&root, &buffer_local);
    //             let mut tree = Tree::new(root.clone());
    //             while !children.is_empty() {

    //             }
    //             let children = get_frame_children(&root, &buffer_local);
                
    //             println!("{}", tree);
    //         }
    //         None => println!("No tree to visualize."),
    //     }

        // buffer_local.iter().for_each(|(name, frame)| {
        //     let _insert_res = updated_buffer.insert(
        //         name.to_string(),
        //         TransformStamped {
        //             time_stamp: Instant::now(),
        //             parent_frame_id: frame.parent_frame_id.clone(),
        //             child_frame_id: frame.child_frame_id.clone(),
        //             transform: frame.transform,
        //             json_metadata: frame.json_metadata.clone(),
        //         },
        //     );
        // });

        // *buffer.lock().unwrap() = updated_buffer;
//         tokio::time::sleep(Duration::from_millis(refresh_rate)).await;
//     }
// }

// fn label<P: AsRef<Path>>(p: P) -> String {
//     p.as_ref().file_name().unwrap().to_str().unwrap().to_owned()
// }

// fn tree<P: AsRef<Path>>(p: P) -> io::Result<Tree<String>> {
//     let result = fs::read_dir(&p)?.filter_map(|e| e.ok()).fold(
//         Tree::new(label(p.as_ref().canonicalize()?)),
//         |mut root, entry| {
//             let dir = entry.metadata().unwrap();
//             if dir.is_dir() {
//                 root.push(tree(entry.path()).unwrap());
//             } else {
//                 root.push(Tree::new(label(entry.path())));
//             }
//             root
//         },
//     );
//     Ok(result)
// }

// fn main() {
//     let dir = env::args().nth(1).unwrap_or_else(|| String::from("."));
//     match tree(dir) {
//         Ok(tree) => println!("{}", tree),
//         Err(err) => println!("error: {}", err),
//     }
// }

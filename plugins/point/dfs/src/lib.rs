use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use std::{
    collections::{HashSet, VecDeque},
    sync::Mutex,
};
use types::XY;

lazy_static! {
    static ref BOUNDS: Mutex<XY> = Mutex::new(XY { x: 1920, y: 1080 });
    static ref QUEUE: Mutex<VecDeque<XY>> = Mutex::new(VecDeque::new());
    static ref QUEUED: Mutex<HashSet<XY>> = Mutex::new(HashSet::new());
    static ref USED: Mutex<HashSet<XY>> = Mutex::new(HashSet::new());
}

#[no_mangle]
pub extern "C" fn set_size(width: usize, height: usize) {
    let mut bounds = BOUNDS.lock().unwrap();
    bounds.x = width;
    bounds.y = height;

    let origin = XY {
        x: width / 2,
        y: height / 2,
    };

    let mut queue = QUEUE.lock().unwrap();
    let mut queued = QUEUED.lock().unwrap();

    queue.push_back(origin);
    queued.insert(origin);
}

#[no_mangle]
pub extern "C" fn get_point() -> XY {
    let bounds = BOUNDS.lock().unwrap();
    let mut queue = QUEUE.lock().unwrap();
    let mut queued = QUEUED.lock().unwrap();
    let mut used = USED.lock().unwrap();

    loop {
        // Next color to return
        let xy = queue.pop_front().unwrap();

        // Add neighbors that aren't already queued or used to queue
        let mut neighbors = Vec::new();
        for x in (1.max(xy.x) - 1)..=(bounds.x - 1).min(xy.x + 1) {
            for y in (1.max(xy.y) - 1)..=(bounds.y - 1).min(xy.y + 1) {
                let neighbor = XY { x, y };
                if !queued.contains(&neighbor) && !used.contains(&neighbor) {
                    neighbors.push(neighbor);
                }
            }
        }
        neighbors.shuffle(&mut rand::thread_rng());

        for neighbor in neighbors {
            queue.push_back(neighbor);
            queued.insert(neighbor);
        }

        used.insert(xy);
        return xy;
    }
}

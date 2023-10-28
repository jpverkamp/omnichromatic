use lazy_static::lazy_static;
use rand::{seq::SliceRandom, Rng};
use std::{collections::HashSet, sync::Mutex};
use types::XY;

lazy_static! {
    static ref BOUNDS: Mutex<XY> = Mutex::new(XY { x: 1920, y: 1080 });
    static ref QUEUE: Mutex<Vec<XY>> = Mutex::new(Vec::new());
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

    queue.push(origin);
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
        // If we have no more queued points, choose a random one

        if queue.is_empty() {
            let x = rand::thread_rng().gen_range(0..bounds.x);
            let y = rand::thread_rng().gen_range(0..bounds.y);
            eprintln!("Random point: {}, {}", x, y);
            queue.push(XY { x, y });
        }
        let xy = *queue.last().expect("at least one value in queue");

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

        // If we have no more neighbors, backtrack
        // Otherwise, choose one randomly
        if neighbors.is_empty() {
            queue.pop();
        } else {
            let neighbor = neighbors
                .choose(&mut rand::thread_rng())
                .expect("at least one neighbor");
            queue.push(*neighbor);
            queued.insert(*neighbor);
        }

        // If we haven't already returned this point, do so now
        if !used.contains(&xy) {
            used.insert(xy);
            return xy;
        }
    }
}

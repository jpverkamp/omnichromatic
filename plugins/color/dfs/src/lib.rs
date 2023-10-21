use lazy_static::lazy_static;
use std::{sync::Mutex, collections::{VecDeque, HashSet}};
use types::{RGB, XY};
use rand::seq::SliceRandom;

lazy_static! {
    // The order to check colors
    static ref QUEUE: Mutex<VecDeque<RGB>> = {
        let mut queue = VecDeque::new();
        queue.push_back(RGB { r: 0, g: 0, b: 0 });
        Mutex::new(queue)
    };

    // The colors that have already queued (faster lookup than VecDeque)
    static ref QUEUED: Mutex<HashSet<RGB>> = {
        let mut queued = HashSet::new();
        queued.insert(RGB { r: 0, g: 0, b: 0 });
        Mutex::new(queued)
    };

    // The colors that have already been returned
    static ref USED: Mutex<HashSet<RGB>> = Mutex::new(HashSet::new());
}

#[no_mangle]
pub extern "C" fn get_color(_pt: XY) -> RGB {
    let mut queue = QUEUE.lock().unwrap();
    let mut queued = QUEUED.lock().unwrap();
    let mut used = USED.lock().unwrap();

    loop { 
        // Next color to return
        let rgb = queue.pop_front().unwrap();

        // Add neighbors that aren't already queued or used to queue
        let mut neighbors = Vec::new();
        for r in (1.max(rgb.r) - 1)..=255.min(rgb.r + 1) {
            for g in (1.max(rgb.g) - 1)..=255.min(rgb.g + 1) {
                for b in (1.max(rgb.b) - 1)..=255.min(rgb.b + 1) {
                    let neighbor = RGB { r, g, b };
                    if !queued.contains(&neighbor) && !used.contains(&neighbor) {
                        neighbors.push(neighbor);
                    }
                }
            }
        }
        neighbors.shuffle(&mut rand::thread_rng());

        for neighbor in neighbors {
            queue.push_back(neighbor);
            queued.insert(neighbor);
        }

        used.insert(rgb);
        return rgb;
    }
}

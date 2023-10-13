use crate::producers::{OrderProducer, RGB, XY};

use image::RgbImage;
use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct BFSOrderProducer {
    width: u32,
    height: u32,
    queue: VecDeque<XY>,
    set: HashSet<XY>,
}

impl BFSOrderProducer {
    pub fn new(width: u32, height: u32) -> Self {
        let origin = [width / 2, height / 2];
        
        let mut queue = VecDeque::new();
        queue.push_front(origin);

        let mut set = HashSet::new();
        set.insert(origin);

        BFSOrderProducer { 
            width,
            height,
            queue,
            set
        }
    }
}

impl OrderProducer for BFSOrderProducer {
    fn to_string(&self) -> String {
        return format!("BFSOrderProducer(width={}, height={})", self.width, self.height);
    }

    fn next(&mut self, _img: &RgbImage, _rgb: Option<RGB>) -> XY {
        loop {
            // The next element in the queue is the one we're returning
            let next = self.queue.pop_front().unwrap();
            let [x, y] = next;

            // Add neighbors we haven't already added
            let mut neighbors = vec![];
            for xd in -1..=1 {
                for yd in -1..=1 {
                    let xn = x as i32 + xd;
                    let yn = y as i32 + yd;

                    if xn < 0 || yn < 0 || xn >= self.width as i32 || yn >= self.height as i32 {
                        continue;
                    }

                    let neighbor = [xn as u32, yn as u32];

                    if self.set.contains(&neighbor) {
                        continue;
                    }

                    neighbors.push(neighbor);
                }
            }
            neighbors.shuffle(&mut rand::thread_rng());

            for neighbor in neighbors.into_iter() {
                self.queue.push_front(neighbor);
                self.set.insert(neighbor);
            }

            return next; 
        }
    }
}
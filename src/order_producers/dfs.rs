use crate::producers::{OrderProducer, RGB, XY};

use image::RgbImage;
use rand::seq::SliceRandom;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct DFSOrderProducer {
    width: u32,
    height: u32,
    queue: Vec<XY>,
    set: HashSet<XY>,
}

impl DFSOrderProducer {
    pub fn new(width: u32, height: u32) -> Self {
        let origin = [width / 2, height / 2];
        
        let mut queue = Vec::new();
        queue.push(origin);

        let set = HashSet::new();

        DFSOrderProducer { 
            width,
            height,
            queue,
            set
        }
    }
}

impl OrderProducer for DFSOrderProducer {
    fn to_string(&self) -> String {
        return format!("DFSOrderProducer(width={}, height={})", self.width, self.height);
    }

    fn next(&mut self, _img: &RgbImage, _rgb: Option<RGB>) -> XY {
        loop {
            let [x, y] = self.queue.last().unwrap();

            // Try neighbors in a random order
            let mut neighbors = vec![];
            for xd in -1..=1 {
                for yd in -1..=1 {
                    let xn = *x as i32 + xd;
                    let yn = *y as i32 + yd;

                    if xn < 0 || yn < 0 || xn >= self.width as i32 || yn >= self.height as i32 {
                        continue;
                    }

                    let next = [xn as u32, yn as u32];

                    if self.set.contains(&next) {
                        continue;
                    }

                    neighbors.push(next);
                }
            }

            // No valid neighbors, backtrack
            if neighbors.len() == 0 {
                self.queue.pop();
                continue;
            }

            // Otherwise, return a random valid neighbor
            let next = neighbors.choose(&mut rand::thread_rng()).unwrap().clone();

            // We found a new pixel that hasn't been used, queue and return it
            self.queue.push(next);
            self.set.insert(next);
            return next; 
        }
    }
}
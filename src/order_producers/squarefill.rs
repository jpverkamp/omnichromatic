use std::collections::{HashSet, VecDeque};

use crate::producers::{OrderProducer, RGB, XY};
use image::RgbImage;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct SquareFillProducer {
    height: u32,
    width: u32,
    queue: VecDeque<XY>,
    queued: HashSet<XY>,
    returned: HashSet<XY>,
}

impl SquareFillProducer {
    pub fn new(width: u32, height: u32) -> Self {
        let origin = [
            rand::thread_rng().gen_range(0..width),
            rand::thread_rng().gen_range(0..height),
        ];

        let mut queue = Vec::new();
        queue.push(origin);

        let mut queued = HashSet::new();
        queued.insert(origin);

        SquareFillProducer {
            width,
            height,
            queue: VecDeque::from(queue),
            queued,
            returned: HashSet::new(),
        }
    }
}

impl OrderProducer for SquareFillProducer {
    fn to_string(&self) -> String {
        format!("SquareFillProducer(size={}x{})", self.width, self.height)
    }

    fn next(&mut self, _img: &RgbImage, _rgb: Option<RGB>) -> XY {
        'next: loop {
            let current = self.queue.pop_front().unwrap();
            let [x, y] = current;

            for xd in (-1 as i32)..1 {
                for yd in (-1 as i32)..1 {
                    let xn = x as i32 + xd;
                    let yn = y as i32 + yd;

                    let mut collision = false;

                    if xn < 0 || xn >= self.width as i32 || yn < 0 || yn >= self.height as i32 {
                        collision = true;
                    }

                    let next = [xn as u32, yn as u32];

                    if self.returned.contains(&next) {
                        collision = true;
                    }

                    // If we ran off the edge or hit a previous pixel, clear queues and generate a new seed
                    if collision {
                        let origin = [
                            rand::thread_rng().gen_range(0..self.width),
                            rand::thread_rng().gen_range(0..self.height),
                        ];

                        self.queue.clear();
                        self.queue.push_back(origin);

                        self.queued.clear();
                        self.queued.insert(origin);

                        continue 'next;
                    }

                    // Don't double queue
                    if self.queued.contains(&next) {
                        continue;
                    }

                    self.queue.push_back(next);
                    self.queued.insert(next);
                }
            }

            for xd in 0.max(x - 1)..(x + 2).min(self.width) {
                for yd in 0.max(y - 1)..(y + 2).min(self.height) {
                    let next = [xd, yd];

                    if self.queued.contains(&next) {
                        continue;
                    }

                    self.queue.push_back(next);
                    self.queued.insert(next);
                }
            }

            return current;
        }
    }
}

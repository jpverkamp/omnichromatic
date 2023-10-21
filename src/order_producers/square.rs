use std::collections::{HashSet, VecDeque};

use crate::producers::{OrderProducer, RGB, XY};
use image::RgbImage;

#[derive(Clone, Debug)]
pub struct SquareProducer {
    height: u32,
    width: u32,
    queue: VecDeque<XY>,
    queued: HashSet<XY>,
}

impl SquareProducer {
    pub fn new(width: u32, height: u32) -> Self {
        let origin = [width / 2, height / 2];

        let mut queue = Vec::new();
        queue.push(origin);

        let mut queued = HashSet::new();
        queued.insert(origin);

        SquareProducer {
            width,
            height,
            queue: VecDeque::from(queue),
            queued,
        }
    }
}

impl OrderProducer for SquareProducer {
    fn to_string(&self) -> String {
        format!("SquareProducer(size={}x{})", self.width, self.height)
    }

    fn next(&mut self, _img: &RgbImage, _rgb: Option<RGB>) -> XY {
        loop {
            let current = self.queue.pop_front().unwrap();
            let [x, y] = current;

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

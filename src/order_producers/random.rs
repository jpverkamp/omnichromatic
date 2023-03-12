use rand::Rng;
use std::collections::HashSet;
use image::RgbImage;
use crate::producers::{OrderProducer, RGB, XY};

#[derive(Clone, Debug)]
pub struct RandomOrderProducer {
    height: u32,
    width: u32,
    returned: HashSet<XY>,
}

impl RandomOrderProducer {
    pub fn new(width: u32, height: u32) -> Self {
        RandomOrderProducer { 
            width,
            height,
            returned: HashSet::new(),
        }
    }
}

impl OrderProducer for RandomOrderProducer {
    fn to_string(&self) -> String {
        format!("RandomOrderProducer(size={}x{})", self.width, self.height)
    }

    fn next(&mut self, _img: &RgbImage, _rgb: Option<RGB>) -> XY {
        loop {
            let xy = [
                rand::thread_rng().gen_range(0..self.width),
                rand::thread_rng().gen_range(0..self.height),
            ];
            if !self.returned.contains(&xy) {
                self.returned.insert(xy);
                return xy;
            }
        }
    }
}
use crate::producers::{ColorProducer, RGB, XY};
use image::RgbImage;
use rand::Rng;
use std::collections::HashSet;

#[derive(Debug)]
pub struct RandomColorProducer {
    pub returned: HashSet<RGB>,
}

impl RandomColorProducer {
    pub fn new() -> Self {
        RandomColorProducer {
            returned: HashSet::new(),
        }
    }
}

impl ColorProducer for RandomColorProducer {
    fn to_string(&self) -> String {
        return format!("RandomColorProducer");
    }

    fn next(&mut self, _img: &RgbImage, _pt: Option<XY>) -> Option<RGB> {
        loop {
            let rgb = [
                rand::thread_rng().gen(),
                rand::thread_rng().gen(),
                rand::thread_rng().gen(),
            ];
            if !self.returned.contains(&rgb) {
                self.returned.insert(rgb);
                return Some(rgb);
            }
        }
    }
}

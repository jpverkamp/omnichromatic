use rand::Rng;
use std::collections::HashSet;
use image::RgbImage;
use crate::producers::{ColorProducer, RGB, XY};

#[derive(Debug)]
pub struct NeighboringOrderedColorProducer {
    stack: Vec<RGB>,
    returned: HashSet<RGB>,
}

impl NeighboringOrderedColorProducer {
    pub fn new() -> Self {
        NeighboringOrderedColorProducer { 
            stack: Vec::new(),
            returned: HashSet::new(),
        }
    }
}

impl ColorProducer for NeighboringOrderedColorProducer {
    fn to_string(&self) -> String {
        return format!("NeighboringOrderedColorProducer");
    }

    fn next(&mut self, _img: &RgbImage, _pt: Option<XY>) -> Option<RGB> {
        loop {
            // Fallback: generate a random color
            let mut rgb = [
                rand::thread_rng().gen(),
                rand::thread_rng().gen(),
                rand::thread_rng().gen()
            ];

            // If we have a color on the stack, pop and try neighbors
            if let Some(prev_rgb) = self.stack.pop() {
                let mut potentials = Vec::new();

                for rd in -1..1 {
                    for gd in -1..1 {
                        for bd in -1..1 {
                            let potential = [
                                (prev_rgb[0] as i32 + rd) as u8,
                                (prev_rgb[1] as i32 + gd) as u8,
                                (prev_rgb[2] as i32 + bd) as u8,
                            ];

                            if self.returned.contains(&potential) {
                                continue;
                            }

                            potentials.push(potential);
                        }
                    }
                }

                if potentials.len() > 0 {
                    self.stack.push(prev_rgb);
                    rgb = potentials[0];
                }
            }

            if !self.returned.contains(&rgb) {
                self.returned.insert(rgb);
                self.stack.push(rgb);
                return Some(rgb);
            }
        }
    }
}
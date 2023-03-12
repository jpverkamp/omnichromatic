use image::RgbImage;
use crate::producers::{ColorProducer, RGB, XY};

#[derive(Clone, Debug)]
pub struct GrayCodeColorProducer {
    pub skip: usize,
    pub index: usize,
}

impl GrayCodeColorProducer {
    pub fn new(width: usize, height: usize) -> Self {
        GrayCodeColorProducer { 
            skip: 256_usize.pow(3) / (width * height),
            index: 0 
        }
    }
}

impl ColorProducer for GrayCodeColorProducer {
    fn to_string(&self) -> String {
        format!("GrayCodeColorProducer(skip={})", self.skip)
    }

    fn next(&mut self, _img: &RgbImage, _pt: Option<XY>) -> Option<RGB> {
        let g = self.index ^ (self.index << 1);
        self.index += self.skip;

        Some([
            (g >> 16) as u8,
            (g >>  8) as u8,
            (g >>  0) as u8,
        ])
    }
}
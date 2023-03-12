use image::RgbImage;
use crate::producers::{ColorProducer, RGB, XY};

#[derive(Debug)]
pub struct OrderedColorProducer {
    pub index: usize
}

impl OrderedColorProducer {
    pub fn new() -> Self {
        OrderedColorProducer { index: 0 }
    }
}

impl ColorProducer for OrderedColorProducer {
    fn to_string(&self) -> String {
        return format!("OrderedColorProducer");
    }
    
    fn next(&mut self, _img: &RgbImage, _pt: Option<XY>) -> Option<RGB> {
        let rgb = [
            (self.index >> 16) as u8,
            (self.index >>  8) as u8,
            (self.index >>  0) as u8,
        ];
        self.index += 1;
        return Some(rgb);
    }
}
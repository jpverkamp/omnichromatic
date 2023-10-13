use crate::producers::{OrderProducer, RGB, XY};
use image::RgbImage;

#[derive(Clone, Debug)]
pub struct WritingOrderProducer {
    width: u32,
    x: u32,
    y: u32,
}

impl WritingOrderProducer {
    pub fn new(width: u32) -> Self {
        WritingOrderProducer { width, x: 0, y: 0 }
    }
}

impl OrderProducer for WritingOrderProducer {
    fn to_string(&self) -> String {
        return format!("WritingOrderProducer(width={})", self.width);
    }

    fn next(&mut self, _img: &RgbImage, _rgb: Option<RGB>) -> XY {
        let xy = [self.x, self.y];
        self.x += 1;
        if self.x >= self.width {
            self.x = 0;
            self.y += 1;
        }
        xy
    }
}

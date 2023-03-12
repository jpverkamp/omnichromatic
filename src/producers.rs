use image::{RgbImage};

pub type XY = [u32; 2];
pub type RGB = [u8; 3];

/*
 * A ColorProducer should produce one RGB color for each call. 
 * 
 * img is a non-mutable reference to the image being built. You can fetch
 * any color that has already been set here, any colors not set will be
 * set to 0,0,0.
 * 
 * This function will be called a maximum of 2^24 times (once for each
 * unique RGB color; the expected size in that case is 4096x4096 but this
 * is not guaranteed). The same color should never be returned from this
 * function.
 * 
 * This function may be called in up to two different locations:
 * - Before the OrderProducer
 * - After the OrderProducer
 * 
 * The first call will always be made with pt as None. If this ColorProducer
 * depends on the point, then return None. This function will then be called
 * again later with pt having a value. 
 */
pub trait ColorProducer {
    fn to_string(&self) -> String;
    fn next(&mut self, img: &RgbImage, pt: Option<XY>) -> Option<RGB>;
}

/*
 * An OrderProducer should produce one X,Y point in the image each call.
 * 
 * img is a non-mutable reference to the image being built. You can fetch
 * any color that has already been set here, any colors not set will be
 * set to 0,0,0.
 * 
 * When it's been called width*height times, it should have produced each 
 * X,Y point exactly once. The maximum this function will be called is 
 * 2^24 times (once for each possible RGB color). 
 * 
 * If the ColorProducer produces a color without a coordinate, then rgb
 * will have that color, otherwise it will be None. 
 */
pub trait OrderProducer {
    fn to_string(&self) -> String;
    fn next(&mut self, img: &RgbImage, rgb: Option<RGB>) -> XY;
}
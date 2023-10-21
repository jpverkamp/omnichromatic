use std::collections::HashSet;

mod producers;
use producers::{ColorProducer, OrderProducer};

mod color_producers;
use color_producers::*;

mod order_producers;
use order_producers::*;

fn main() {
    let width = 1024;
    let height = 1024;

    // let width = 4096;
    // let height = 4096;

    let mut image = image::RgbImage::new(width, height);

    let color_producers: Vec<Box<dyn ColorProducer>> = vec![
        Box::new(RandomColorProducer::new()),
        Box::new(OrderedColorProducer::new()),
        Box::new(GrayCodeColorProducer::new(width as usize, height as usize)),
        Box::new(NeighboringOrderedColorProducer::new()),
        Box::new(NeighboringRandomColorProducer::new()),
    ];

    for mut color_producer in color_producers {
        let order_producers: Vec<Box<dyn OrderProducer>> = vec![
            // Box::new(WritingOrderProducer::new(width)),
            // Box::new(RandomOrderProducer::new(width, height)),
            // Box::new(DFSOrderProducer::new(width, height)),
            // Box::new(BFSOrderProducer::new(width, height)),
            // Box::new(SquareProducer::new(width, height)),
            Box::new(SquareFillProducer::new(width, height)),
        ];

        for mut order_producer in order_producers {
            println!(
                "{} + {}",
                color_producer.to_string(),
                order_producer.to_string()
            );

            let mut colors_used = HashSet::new();
            let mut points_used = HashSet::new();

            for _i in 0..(width * height) {
                // Try to generate the color first
                let mut rgb = color_producer.next(&image, None);

                // Then the point/order
                let xy = order_producer.next(&image, rgb);

                // If we didn't produce a color already, do so now
                if rgb.is_none() {
                    rgb = color_producer.next(&image, Some(xy));
                }

                // Verify we actually got a color this time and unpack it
                if rgb.is_none() {
                    panic!(
                        "{} did not produce a color after either call",
                        color_producer.to_string()
                    );
                }
                let rgb = rgb.unwrap();

                // Verify no duplicates
                if colors_used.contains(&rgb) {
                    panic!("color {rgb:?} used twice");
                } else {
                    colors_used.insert(rgb);
                }

                if points_used.contains(&xy) {
                    panic!("point {xy:?} used twice");
                } else {
                    points_used.insert(xy);
                }

                let [x, y] = xy;
                let pixel = image.get_pixel_mut(x as u32, y as u32);
                *pixel = image::Rgb::from(rgb);
            }

            let filename = format!(
                "output/{}x{} - {} - {}.png",
                width,
                height,
                color_producer.to_string(),
                order_producer.to_string()
            );
            image.save(filename).unwrap();
        }
    }
}

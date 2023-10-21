use libloading::Library;
use providers::*;
use std::collections::HashSet;

mod providers;

fn main() {
    env_logger::init();

    let point_provider_lib =
        unsafe { Library::new("target/release/libpoint_writing.dylib").unwrap() };
    let point_provider = PointProvider::new(&point_provider_lib);

    let color_provider_lib =
        unsafe { Library::new("target/release/libcolor_ordered.dylib").unwrap() };
    let color_provider = ColorProvider::new(&color_provider_lib);

    let width = 640;
    let height = 480;
    let size = width * height;

    point_provider.set_size(width, height);

    let mut image = image::RgbImage::new(width as u32, height as u32);

    let mut points_used = HashSet::new();
    let mut colors_used = HashSet::new();

    for i in 0..size {
        let pt = point_provider.next();
        let rgb = color_provider.next(pt);

        assert!(pt.x < width);
        assert!(pt.y < height);
        assert!(!points_used.contains(&pt));
        assert!(!colors_used.contains(&rgb));

        if i % 10000 == 0 {
            log::info!("[{i} of {size}] {rgb} @ {pt}");
        } else {
            log::debug!("[{i} of {size}] {rgb} @ {pt}");
        }

        image.put_pixel(pt.x as u32, pt.y as u32, image::Rgb([rgb.r, rgb.g, rgb.b]));

        points_used.insert(pt);
        colors_used.insert(rgb);
    }

    image.save("random.png").unwrap();
}

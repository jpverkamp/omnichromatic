use std::collections::HashSet;

use libloading::{Library, Symbol};
use types::{XY, RGB};

type PointProviderFn = fn() -> XY;
type ColorProviderFn = fn(XY) -> RGB;

struct PointProvider<'lib> { 
    set_size: Option<Symbol<'lib, fn(usize, usize)>>,
    get_point: Symbol<'lib, PointProviderFn>,
}

impl<'lib> PointProvider<'lib> { 
    fn new(library: &'lib Library) -> Self { 
        unsafe { 
            let set_size: Option<Symbol<'lib, fn(usize, usize)>> = library.get(b"set_size").ok();
            let get_point: Symbol<'lib, PointProviderFn> = library.get(b"get_point").unwrap();
            PointProvider { set_size, get_point }
        }
    }

    fn set_size(&self, width: usize, height: usize) { 
        if let Some(set_size) = &self.set_size { 
            set_size(width, height);
        }
    }

    fn next(&self) -> XY { 
        (self.get_point)()
    }
}

struct ColorProvider<'lib> { 
    get_color: Symbol<'lib, ColorProviderFn>,
}

impl<'lib> ColorProvider<'lib> { 
    fn new(library: &'lib Library) -> Self { 
        unsafe { 
            let get_color: Symbol<'lib, ColorProviderFn> = library.get(b"get_color").unwrap();
            ColorProvider { get_color }
        }
    }

    fn next(&self, pt: XY) -> RGB { 
        (self.get_color)(pt)
    }
}

fn main() {
    env_logger::init();

    let point_provider_lib = unsafe { Library::new("target/release/libpoint_writing.dylib").unwrap() };
    let point_provider = PointProvider::new(&point_provider_lib);

    let color_provider_lib = unsafe { Library::new("target/release/libcolor_ordered.dylib").unwrap() };
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

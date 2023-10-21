use lazy_static::lazy_static;
use std::sync::Mutex;
use types::{RGB, XY};

lazy_static! {
    static ref CURRENT: Mutex<RGB> = Mutex::new(RGB { r: 0, g: 0, b: 0 });
}

#[no_mangle]
pub extern "C" fn get_color(_pt: XY) -> RGB {
    let mut current = CURRENT.lock().unwrap();
    let rgb = RGB {
        r: current.r,
        g: current.g,
        b: current.b,
    };

    if current.r == 255 {
        current.r = 0;
        if current.g == 255 {
            current.g = 0;
            if current.b == 255 {
                current.b = 0;
            } else {
                current.b += 1;
            }
        } else {
            current.g += 1;
        }
    } else {
        current.r += 1;
    }

    rgb
}

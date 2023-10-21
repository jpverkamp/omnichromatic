use lazy_static::lazy_static;
use std::sync::Mutex;
use types::XY;

lazy_static! {
    static ref BOUNDS: Mutex<XY> = Mutex::new(XY { x: 1920, y: 1080 });
    static ref CURRENT: Mutex<XY> = Mutex::new(XY { x: 0, y: 0 });
}

#[no_mangle]
pub extern "C" fn set_size(width: usize, height: usize) {
    let mut bounds = BOUNDS.lock().unwrap();
    bounds.x = width;
    bounds.y = height;
}

#[no_mangle]
pub extern "C" fn get_point() -> XY {
    let bounds = BOUNDS.lock().unwrap();
    let mut current = CURRENT.lock().unwrap();

    let xy = XY {
        x: current.x,
        y: current.y,
    };

    current.x += 1;
    if current.x >= bounds.x {
        current.x = 0;
        current.y += 1;
    }

    xy
}

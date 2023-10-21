use std::collections::HashSet;
use lazy_static::lazy_static;
use std::sync::Mutex;
use types::XY;

lazy_static! {
    static ref BOUNDS: Mutex<XY> = Mutex::new(XY { x: 1920, y: 1080 });
    static ref USED: Mutex<HashSet<XY>> = Mutex::new(HashSet::new());
}

#[no_mangle]
pub extern "C" fn set_size(width: usize, height: usize) {
    let mut bounds = BOUNDS.lock().unwrap();
    bounds.x = width;
    bounds.y = height;
}

#[no_mangle]
pub extern "C" fn get_point() -> XY {
    loop {
        let bounds = BOUNDS.lock().unwrap();

        let x = rand::random::<usize>() % bounds.x;
        let y = rand::random::<usize>() % bounds.y;
        let xy = XY { x, y };

        let mut used = USED.lock().unwrap();
        if !used.contains(&xy) {
            used.insert(xy);
            return xy;
        }
    }
}

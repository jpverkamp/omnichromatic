use std::collections::HashSet;
use lazy_static::lazy_static;
use std::sync::Mutex;
use types::{XY, RGB};

lazy_static! {
    static ref USED: Mutex<HashSet<RGB>> = Mutex::new(HashSet::new());
}


#[no_mangle]
pub extern "C" fn get_color(_pt: XY) -> RGB {
    loop {
        let r = rand::random::<u8>();
        let g = rand::random::<u8>();
        let b = rand::random::<u8>();
        let rgb = RGB { r, g, b };

        let mut used = USED.lock().unwrap();
        if !used.contains(&rgb) {
            used.insert(rgb);
            return rgb;
        }
    }
}

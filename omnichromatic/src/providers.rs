use libloading::{Library, Symbol};
use types::{RGB, XY};

type SetSizeFn = fn(usize, usize);
type PointProviderFn = fn() -> XY;
type ColorProviderFn = fn(XY) -> RGB;

pub struct PointProvider<'lib> {
    set_size: Option<Symbol<'lib, SetSizeFn>>,
    get_point: Symbol<'lib, PointProviderFn>,
}

impl<'lib> PointProvider<'lib> {
    pub fn new(library: &'lib Library) -> Self {
        unsafe {
            let set_size: Option<Symbol<'lib, SetSizeFn>> = library.get(b"set_size").ok();
            let get_point: Symbol<'lib, PointProviderFn> = library.get(b"get_point").unwrap();
            PointProvider {
                set_size,
                get_point,
            }
        }
    }

    pub fn set_size(&self, width: usize, height: usize) {
        if let Some(set_size) = &self.set_size {
            set_size(width, height);
        }
    }

    pub fn next(&self) -> XY {
        (self.get_point)()
    }
}

pub struct ColorProvider<'lib> {
    get_color: Symbol<'lib, ColorProviderFn>,
}

impl<'lib> ColorProvider<'lib> {
    pub fn new(library: &'lib Library) -> Self {
        unsafe {
            let get_color: Symbol<'lib, ColorProviderFn> = library.get(b"get_color").unwrap();
            ColorProvider { get_color }
        }
    }

    pub fn next(&self, pt: XY) -> RGB {
        (self.get_color)(pt)
    }
}

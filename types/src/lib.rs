use std::fmt::{Display, Formatter};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct XY {
    pub x: usize,
    pub y: usize,
}

impl Display for XY {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Display for RGB {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "rgb({}, {}, {})", self.r, self.g, self.b)
    }
}

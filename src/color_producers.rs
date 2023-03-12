mod gray_code;
mod neighboring_ordered;
mod neighboring_random;
mod ordered;
mod random;

pub use gray_code::GrayCodeColorProducer;
pub use neighboring_ordered::NeighboringOrderedColorProducer;
pub use neighboring_random::NeighboringRandomColorProducer;
pub use ordered::OrderedColorProducer;
pub use random::RandomColorProducer;
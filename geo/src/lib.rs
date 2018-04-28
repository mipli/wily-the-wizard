extern crate serde;
#[macro_use] extern crate serde_derive;

mod point;
mod rect;

pub use point::Point;
pub use rect::Rect;

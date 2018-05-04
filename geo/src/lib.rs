extern crate serde;
#[macro_use] extern crate serde_derive;

mod point;
mod rect;

pub use point::Point;
pub use rect::Rect;

pub fn get_neigbours(x: i32, y: i32, only_cardinal: bool) -> Vec<Point> {
    let mut cells = vec![];
    let dirs = if only_cardinal {
        vec![(0, -1), (-1, 0), (1, 0), (0, 1)]
    } else {
        vec![(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)]
    };

    for dir in dirs {
        let p: Point = (x + dir.0, y + dir.1).into();
        cells.push(p);
    }
    cells
}

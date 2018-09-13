use rand::{Rng};

use geo::{Point};
use super::Map;

#[derive(Debug)]
enum Orientation {
    Up,
    Down,
    Left,
    Right
}

impl Orientation {
    fn from_doorway(doorway: Point, map: &RoomsyMap) -> Option<Orientation> {
        let left = map.map.get(doorway.x - 1, doorway.y)?;
        if left == 1 {
            return Some(Orientation::Right);
        }
        let right = map.map.get(doorway.x + 1, doorway.y)?;
        if right == 1 {
            return Some(Orientation::Left);
        }
        let up = map.map.get(doorway.x, doorway.y - 1)?;
        if up == 1 {
            return Some(Orientation::Down);
        }
        let down = map.map.get(doorway.x, doorway.y + 1)?;
        if down == 1 {
            return Some(Orientation::Up);
        }
        None
    }
}

#[derive(Debug)]
struct RoomsyMap {
    map: Map,
    doorways: Vec<Point>
}

impl RoomsyMap {
    fn new(width: i32, height: i32) -> Self {
        RoomsyMap {
            map: Map::new(width, height),
            doorways: vec![]
        }
    }
}

fn add_left_room(doorway: Point, width: i32, height: i32, map: &mut RoomsyMap) {
    let left = doorway.x - width;
    let top = doorway.y - 3;
    for x in left..doorway.x {
        for y in top..(top + width) {
            map.map.set(x, y, 1);
        }
    }
    add_horizontal_doorways(top - 1, left, width, map);
    add_horizontal_doorways(top + height, left, width, map);
    add_vertical_doorways(left - 1, top, height, map);
    add_vertical_doorways(left + width, top, height, map);
}

fn add_horizontal_doorways(y: i32, left: i32, width: i32, map: &mut RoomsyMap) {
    for x in left..(left + width) {
        map.map.set(x, y, 2);
        map.doorways.push(Point::new(x, y));
    }
}

fn add_vertical_doorways(x: i32, top: i32, height: i32, map: &mut RoomsyMap) {
    for y in top..(top + height) {
        map.map.set(x, y, 3);
        map.doorways.push(Point::new(x, y));
    }
}

fn add_room<T: Rng>(map: &mut RoomsyMap, rng: &mut T) {
    let mut doorways = vec![];
    for x in 0..map.map.width {
        for y in 0..map.map.height {
            if map.map.get(x, y) == Some(2) {
                doorways.push(Point::new(x, y));
            }
        }
    }

    match rng.choose(doorways.as_slice()) {
        Some(doorway) => {
            let width = 5;
            let height = 5;
            match Orientation::from_doorway(*doorway, map) {
                Some(orientation) => {
                    match orientation {
                        Orientation::Left => add_left_room(*doorway, width, height, map),
                        _ => {}
                    }
                },
                None => {}
            }
        },
        None => {}
    }
}

fn seed(map: &mut RoomsyMap) {
    let x = map.map.width / 2;
    let y = map.map.height / 2;
    for i in x-1..=x+1 {
        for j in y-3..=y+3 {
            map.map.set(i, j, 1);
        }
    }
    map.map.set(x-2, y, 2);
    map.doorways.push(Point::new(x-2, y));
}

pub fn generate<T: Rng>(width: i32, height: i32, min_size: i32, rng: &mut T) -> Map {
    let mut map = RoomsyMap::new(width, height);
    seed(&mut map);
    add_room(&mut map, rng);
    map.map
}

use rand::{Rng};

use geo::{Rect, Point, get_neigbours};
use super::map::Map;
use super::bsp::{Leaf, carve, split};

struct ConnectionMap {
    pub area_count: u32,
    pub current: u32,
    pub map: Map
}

impl ConnectionMap {
    fn new(base_map: &Map) -> Self {
        let mut map = base_map.clone();
        for i in 1..=map.rooms.len() {
            let room = map.rooms[i - 1];
            for x in 0..room.width {
                for y in 0..room.height {
                    map.set(x + room.x1, y + room.y1, i as i32);
                }
            }
        }
        ConnectionMap {
            area_count: map.rooms.len() as u32,
            current: 1,
            map
        }
    }

    fn iter<T: Rng>(&mut self, rng: &mut T) {
        let mut connection_coords: Vec<(i32, i32, i32)> = vec![];
        for x in 0..self.map.width {
            for y in 0..self.map.height {
                if self.map.get(x, y) == Some(0) {
                    let neighbours = get_neigbours(x as i32, y as i32, true);
                    let connections = neighbours.iter().filter_map(|pos| {
                        match self.map.get(pos.x, pos.y) {
                            Some(v) if v != 0 => Some(v),
                            _ => None
                        }
                    }).collect::<Vec<_>>();
                    if connections.len() == 2 {
                        let has_one_self = connections.iter().filter(|v| **v == 1).collect::<Vec<_>>().len() == 1;
                        let fst = connections[0];
                        let snd = connections[1];
                        if has_one_self {
                            if fst == 1 {
                                connection_coords.push((x, y, snd));
                            } else {
                                connection_coords.push((x, y, fst));
                            }
                        }
                    }
                }
            }
        }
        match rng.choose(connection_coords.as_slice()) {
            Some((x, y, v)) => {
                self.map.set(*x, *y, 1);
                self.map.doors.push(Point::new(*x, *y));
                for i in 0..self.map.width {
                    for j in 0..self.map.height {
                        if self.map.get(i, j) == Some(*v) {
                            self.map.set(i, j, 1);
                        }
                    }
                }
            },
            None => {}
        }
        if rng.gen::<f32>() < 0.6 {
            match rng.choose(connection_coords.as_slice()) {
                Some((x, y, v)) => {
                    let neighbours = get_neigbours(*x, *y, true);
                    let connections = neighbours.iter().filter_map(|pos| {
                        match self.map.get(pos.x, pos.y) {
                            Some(v) if v != 0 => Some(v),
                            _ => None
                        }
                    }).collect::<Vec<_>>();
                    if connections.len() == 2 {
                        self.map.set(*x, *y, 1);
                        self.map.doors.push(Point::new(*x, *y));
                        for i in 0..self.map.width {
                            for j in 0..self.map.height {
                                if self.map.get(i, j) == Some(*v) {
                                    self.map.set(i, j, 1);
                                }
                            }
                        }
                    }
                },
                None => {}
            }
        }
    }
}

fn create_rooms(leaf: &mut Leaf) {
    let mut created_room = false;
    if let Some(ref mut left) = *leaf.left {
        create_rooms(left);
        created_room = true;
    }
    if let Some(ref mut right) = *leaf.right {
        create_rooms(right);
        created_room = true;
    }
    if !created_room && leaf.room.is_none() {
        let (x, width) = if leaf.dim.x1 == 0 {
            (0, leaf.dim.width)
        } else {
            (leaf.dim.x1 + 1, leaf.dim.width - 1)
        };
        let (y, height) = if leaf.dim.y1 == 0 {
            (0, leaf.dim.height)
        } else {
            (leaf.dim.y1 + 1, leaf.dim.height - 1)
        };
        leaf.room = Some(Rect::new(x, y, width, height));
    }
}

pub fn generate<T: Rng>(width: i32, height: i32, min_size: i32, rng: &mut T) -> Map {
    let mut root = Leaf::new(0, 0, width - 2, height - 2);
    split(&mut root, min_size, rng);
    create_rooms(&mut root);
    let mut map = Map::new(width - 2, height - 2);
    let _ = carve(&root, &mut map, false, rng);
    let padded_map = Map::pad_map(map);
    let mut connection_map = ConnectionMap::new(&padded_map);
    for _ in 0..connection_map.area_count {
        connection_map.iter(rng);
    }
    let mut tower_map = connection_map.map;
    tower_map.stairs = Some(tower_map.rooms[tower_map.rooms.len() - 1].center());
    tower_map
}

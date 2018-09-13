use rand::{Rng};
use std::collections::{HashSet};

use geo::{Point, Rect, get_neigbours};
use super::Map;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SpawnDirection {
    Up,
    Down,
    Left,
    Right
}

struct Generator<'a, T: Rng + 'a> {
    rng: &'a mut T,
    map: Map,
    possible_connections: Vec<(Point, SpawnDirection)>
}

fn is_possible_connection(map: &Map, point: &Point) -> bool {
    let neighbours = get_neigbours(point.x, point.y, true);
    neighbours.iter().filter(|p| {
        map.get(p.x, p.y) == Some(1)
    }).count() == 1
}

impl<'a, T: Rng> Generator<'a, T> {
    pub fn new(rng: &'a mut T, map: Map) -> Self {
        Self {
            rng,
            map,
            possible_connections: vec![]
        }
    }

    fn is_possible_connection(&self, point: &Point) -> bool {
        let neighbours = get_neigbours(point.x, point.y, true);
        neighbours.iter().filter(|p| {
            self.map.get(p.x, p.y) == Some(1)
        }).count() == 1
    }

    pub fn next_room(&mut self) -> Option<(Point, Rect)> {
        if self.possible_connections.len() == 1 {
            return None;
        }
        let index = self.rng.gen_range(0, self.possible_connections.len());
        let (spawn, direction) = self.possible_connections.remove(index);
        let width = self.rng.gen_range(4, 8);
        let height = self.rng.gen_range(4, 8);
        let mut room = Rect::new(0, 0, width, height);
        let point = match direction {
            SpawnDirection::Up => {
                Point::new(spawn.x - room.width / 2, spawn.y - room.height)
            },
            SpawnDirection::Down => {
                Point::new(spawn.x - room.width / 2, spawn.y + 1)
            },
            SpawnDirection::Left => {
                Point::new(spawn.x - room.width, spawn.y - room.height / 2)
            },
            SpawnDirection::Right => {
                Point::new(spawn.x + 1, spawn.y - room.height / 2)
            }
        };
        room.move_to(point);
        Some((spawn, room.clone()))
    }

    pub fn can_fit(&self, room: &Rect) -> bool {
        if !self.map.in_bounds(room.x1 - 1, room.y1 - 1) || !self.map.in_bounds(room.x2 + 1, room.y2 + 1) {
            return false;
        }
        let mut valid = true;
        for x in (room.x1 - 1)..=room.x2 {
            for y in (room.y1 - 1)..=room.y2 {
                valid = valid && self.map.get(x, y) == Some(0);
            }
        }
        valid
    }

    pub fn carve_room(&mut self, room: Rect) {
        for x in room.x1..room.x2 {
            for y in room.y1..room.y2 {
                self.map.set(x, y, 1);
            }
        }


        for x in room.x1-1..=room.x2 {
            let point = Point::new(x, room.y1 - 1);
            if self.is_possible_connection(&point) {
                self.possible_connections.push((point, SpawnDirection::Up));
            }
            let point = Point::new(x, room.y2);
            if self.is_possible_connection(&point) {
                self.possible_connections.push((point, SpawnDirection::Down));
            }
        }
        for y in room.y1-1..=room.y2 {
            let point = Point::new(room.x1 - 1, y);
            if self.is_possible_connection(&point) {
                self.possible_connections.push((point, SpawnDirection::Left));
            }
            let point = Point::new(room.x2, y);
            if self.is_possible_connection(&point) {
                self.possible_connections.push((point, SpawnDirection::Right));
            }
        }
        self.map.rooms.push(room);
    }

    pub fn grow_room(&self, room: &mut Rect, f: impl Fn(&mut Rect, i32) -> Result<(), ()>) {
        let bound = 5;
        let mut last_valid = None;
        let mut should_grow = false;
        for i in 1..=bound {
            let mut r = room.clone();
            if f(&mut r, i).is_ok() {
                if self.can_fit(&r) {
                    last_valid = Some(i);
                } else {
                    should_grow = true;
                }
            }
        }
        if should_grow {
            if let Some(g) = last_valid {
                let _ = f(room, g);
            }
        }
    }

    pub fn clean_connections(&mut self) -> Vec<Point> {
        let map = &self.map;
        let t = self.possible_connections
            .drain_filter(|(p, _)| !is_possible_connection(map, p))
            .map(|(p, _)| p)
            .collect::<Vec<_>>();
        t
    }

    pub fn place_stairs(&mut self) {
        let start = self.map.rooms[0].center();
        let mut stairs = start;
        self.map.rooms.iter().skip(1).for_each(|r| {
            if get_doors(r, &self.map).len() == 1 && r.center().distance(start) > stairs.distance(start) {
                stairs = r.center();
            }
        });
        self.map.stairs = Some(stairs);
    }

    pub fn add_doors(&mut self) {
        let mut doors = HashSet::new();
        self.map.rooms.iter().for_each(|r| {
            let rd = get_doors(r, &self.map);
            rd.iter().for_each(|d| {
                doors.insert(d.clone());
            });
        });
        self.map.doors = doors.into_iter().collect::<Vec<_>>();

    }
}

pub fn get_doors(room: &Rect, map: &Map) -> Vec<Point> {
    let mut doors = vec![];
    for x in room.x1-1..=room.x2 {
        if map.get(x, room.y1 - 1) == Some(1) {
            doors.push((x, room.y1 - 1).into())
        }
        if map.get(x, room.y2) == Some(1) {
            doors.push((x, room.y2).into())
        }
    }
    for y in room.y1-1..=room.y2 {
        if map.get(room.x1 - 1, y) == Some(1) {
            doors.push((room.x1 - 1, y).into())
        }
        if map.get(room.x2, y) == Some(1) {
            doors.push((room.x2, y).into())
        }
    }
    doors
}

pub fn generate<T: Rng>(width: i32, height: i32, _min_size: i32, rng: &mut T) -> Map {
    let map = Map::new(width, height);
    let mut generator = Generator::new(rng, map);
    generator.carve_room(Rect::new(10, 20, 30, 5));
    while let Some((spawn, mut room))= generator.next_room() {
        if generator.can_fit(&room) {
            // growing right
            generator.grow_room(&mut room, |r, s| {
                r.grow(Point::new(s, 0))
            });
            // growing left
            generator.grow_room(&mut room, |r, s| {
                let org = Point::new(r.x1 - s, r.y1);
                r.move_to(org);
                r.grow(Point::new(s, 0))
            });
            // growing down
            generator.grow_room(&mut room, |r, s| {
                r.grow(Point::new(0, s))
            });
            // growing up
            generator.grow_room(&mut room, |r, s| {
                let org = Point::new(r.x1, r.y1 - s);
                r.move_to(org);
                r.grow(Point::new(0, s))
            });
            generator.carve_room(room);
            generator.map.set(spawn.x, spawn.y, 1);
            let impossible_connections = generator.clean_connections();

            impossible_connections.iter().for_each(|p| {
                if generator.rng.gen::<f32>() < 0.1 {
                    if get_neigbours(p.x, p.y, true)
                        .iter()
                        .filter(|p| generator.map.get(p.x, p.y) == Some(1))
                        .count() == 2 {
                        if generator.map.get(p.x - 1, p.y) == generator.map.get(p.x + 1, p.y) {
                            generator.map.set(p.x, p.y, 1);
                        }
                    }
                }
            });
        }
    }
    generator.place_stairs();
    generator.add_doors();

    generator.map
}

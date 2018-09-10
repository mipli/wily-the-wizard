use rand::{Rng};

use geo::{Point, Rect, get_neigbours};
use map::Map;

struct Generator {
    map: Map,
    possible_connections: Vec<Point>
}

impl Generator {
    pub fn new(map: Map) -> Self {
        Self {
            map,
            possible_connections: vec![]
        }
    }

    fn is_possible_connection(&self, point: &Point) -> bool {
        let neighbours = get_neigbours(point.x, point.y, true);
        neighbours.iter().filter(|p| {
            match self.map.get(p.x, p.y) {
                Some(m) => m == 1,
                None => false
            }
        }).count() == 1
    }

    pub fn carve_room(&mut self, room: Rect) {
        for x in room.x1..room.x2 {
            for y in room.y1..room.y2 {
                self.map.set(x, y, 1);
            }
        }
        self.possible_connections = self.possible_connections
            .iter()
            .filter(|p| self.is_possible_connection(p))
            .map(|p| *p)
            .collect();


        for x in room.x1-1..=room.x2 {
            let point = Point::new(x, room.y1 - 1);
            if self.is_possible_connection(&point) {
                self.possible_connections.push(point);
            }
            let point = Point::new(x, room.y2);
            if self.is_possible_connection(&point) {
                self.possible_connections.push(point);
            }
        }
        for y in room.y1-1..=room.y2 {
            let point = Point::new(room.x1 - 1, y);
            if self.is_possible_connection(&point) {
                self.possible_connections.push(point);
            }
            let point = Point::new(room.x2, y);
            if self.is_possible_connection(&point) {
                self.possible_connections.push(point);
            }
        }
        self.map.rooms.push(room);
    }
}

/*
fn seed(width: i32, height: i32) -> Map {
    carve_room(&mut map, Rect::new(10, 20, 30, 5));
    map
}
*/

pub fn generate<T: Rng>(width: i32, height: i32, _min_size: i32, _rng: &mut T) -> Map {
    let map = Map::new(width, height);
    let mut generator = Generator::new(map);
    generator.carve_room(Rect::new(10, 20, 30, 5));
    println!("cons: {:?}", generator.possible_connections.len());
    /*
    let map = seed(width, height);
    let mut possible_connections = vec![];
    for x in 9..=40 {
        possible_connections.push(Point::new(x, 19));
        possible_connections.push(Point::new(x, 25));
    }
    for y in 20..25 {
        possible_connections.push(Point::new(9, y));
        possible_connections.push(Point::new(40, y));
    }

    possible_connections.iter().for_each(|p| {
        map.set(p.x, p.y, 1);
    });
    */

    generator.map
}

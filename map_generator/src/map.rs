use std::fmt;
use geo::{Rect, Point};

#[derive(Clone, PartialEq)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub stairs: Option<Point>,
    pub data: Vec<i32>,
    pub rooms: Vec<Rect>,
    pub doors: Vec<Point>
}

impl Map {
    pub fn new(width: i32, height: i32) -> Map {
        let mut data: Vec<i32> = vec![];
        data.resize((width * height) as usize, 0);
        Map {
            width,
            height,
            data,
            stairs: None,
            rooms: vec![],
            doors: vec![]
        }
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    pub fn set_stairs(&mut self, pos: Point) {
        self.stairs = Some(pos);
    }

    pub fn pad_map(map: Map) -> Map {
        let mut data: Vec<i32> = vec![];
        data.resize(((map.width + 2) * (map.height + 2)) as usize, 0);
        for x in 0..map.width {
            for y in 0..map.height {
                match map.get(x, y) {
                    Some(v) => data[((y + 1) * (map.width + 2) + (x + 1)) as usize] = v,
                    None => {}
                }
            }
        }
        let rooms = map.rooms.iter().map(|room| {
            Rect::new(room.x1 + 1, room.y1 + 1, room.width, room.height)
        }).collect();
        let doors = map.doors.iter().map(|door| {
            Point::new(door.x + 1, door.y + 1)
        }).collect();
        Map {
            width: map.width + 2,
            height: map.height + 2,
            stairs: map.stairs,
            data,
            rooms,
            doors,
        }
    }

    fn to_index(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn add_room(&mut self, room: Rect) {
        self.rooms.push(room);
    }

    pub fn get(&self, x: i32, y: i32) -> Option<i32> {
        let index = self.to_index(x, y);
        if index < self.data.len() {
            Some(self.data[self.to_index(x, y)])
        } else {
            None
        }
    }

    pub fn set(&mut self, x: i32, y: i32, v: i32) {
        let index = self.to_index(x, y);
        self.data[index] = v;
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::char;
        let mut map = "".to_string();
        for y in 0..self.height {
            map = format!("{}\n", map);
            for x in 0..self.width {
                let c = self.data[self.to_index(x, y)];
                if c == 0 {
                    map = format!("{}.", map);
                } else if c < 10 {
                    map = format!("{}{}", map, c);
                } else if c < 37 {
                    match char::from_u32((c + 55) as u32) {
                        Some(d) => map = format!("{}{}", map, d),
                        None => map = format!("{}?", map)
                    }
                } else {
                    map = format!("{}?", map);
                }
            }
        }
        write!(f, "{}", map)
    }
}

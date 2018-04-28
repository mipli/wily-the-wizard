use geo::Rect;

#[derive(Debug, Clone, PartialEq)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub data: Vec<i32>,
    pub rooms: Vec<Rect>
}

impl Map {
    pub fn new(width: i32, height: i32) -> Map {
        let mut data: Vec<i32> = vec![];
        data.resize((width * height) as usize, 0);
        Map {
            width,
            height,
            data,
            rooms: vec![]
        }
    }

    pub fn pad_map(map: &Map) -> Map {
        let mut data: Vec<i32> = vec![];
        data.resize(((map.width + 2) * (map.height + 2)) as usize, 0);
        for x in 0..map.width {
            for y in 0..map.height {
                data[((y + 1) * (map.width + 2) + (x + 1)) as usize] = map.get(x, y);
            }
        }
        Map {
            width: map.width + 2,
            height: map.height + 2,
            data,
            rooms: map.rooms.clone()
        }
    }

    fn to_index(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn add_room(&mut self, room: Rect) {
        self.rooms.push(room);
    }

    pub fn get(&self, x: i32, y: i32) -> i32 {
        self.data[self.to_index(x, y)]
    }

    pub fn set(&mut self, x: i32, y: i32, v: i32) {
        let index = self.to_index(x, y);
        self.data[index] = v;
    }
}

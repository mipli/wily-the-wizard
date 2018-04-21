use rand::*;
use point::*;
use std::cmp::{max, min};

pub fn generate_map(width: i32, height: i32) -> (Vec<Vec<i32>>, Vec<Rect>) {
    let mut map = vec![vec![0; height as usize]; width as usize];

    let max_rooms = 50;

    let mut rooms: Vec<Rect>= vec![];

    for _ in 0..max_rooms {
        let room = generate_room(width, height, 4, 8);
        let failed = rooms.iter().any(|r| room.intersect(r));
        if !failed {
            for x in room.x1..room.x2 {
                for y in room.y1..room.y2 {
                    map[x as usize][y as usize] = 1;
                }
            }
            if !rooms.is_empty() {
                let start = room.center();
                let end = rooms[rooms.len() - 1].center();
                carve_tunnel(start, end, &mut map);
            }
            rooms.push(room);
        }
    }

    (map, rooms)
}

fn generate_room(width: i32, height: i32, min_size: i32, max_size: i32) -> Rect {
    let w = thread_rng().gen_range(min_size, max_size);
    let h = thread_rng().gen_range(min_size, max_size);
    Rect::new(thread_rng().gen_range(1, width - w), thread_rng().gen_range(1, height - h), w, h)
}

fn carve_tunnel(start: Point, end: Point, map: &mut Vec<Vec<i32>>) {
    if random() {
        carve_h_tunnel(start.x, start.y, end.x, map);
        carve_v_tunnel(end.x, end.y, start.y, map);
    } else {
        carve_v_tunnel(start.x, start.y, end.y, map);
        carve_h_tunnel(end.x, end.y, start.x, map);
    }
}

fn carve_h_tunnel(x1: i32, y: i32, x2: i32, map: &mut Vec<Vec<i32>>) {
    for x in min(x1, x2)..(max(x1, x2) + 1) {
        map[x as usize][y as usize] = 1;
    }
}

fn carve_v_tunnel(x: i32, y1: i32, y2: i32, map: &mut Vec<Vec<i32>>) {
    for y in min(y1, y2)..(max(y1, y2) + 1) {
        map[x as usize][y as usize] = 1;
    }
}

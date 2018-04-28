use std::cmp::{min, max};
use rand;
use rand::{Rng, XorShiftRng, SeedableRng};

use map::{Rect, Map};

#[derive(Debug)]
struct Leaf {
    dim: Rect,
    left: Box<Option<Leaf>>,
    right: Box<Option<Leaf>>,
    vert: bool,
    room: Option<Rect>
}

impl Leaf {
    fn new(x: i32, y: i32, width: i32, height: i32) -> Leaf {
        Leaf {
            dim: Rect{x, y, width, height},
            left: Box::new(None),
            right: Box::new(None),
            vert: false,
            room: None
        }
    }

    fn split<T: Rng>(&mut self, min_size: i32, rng: &mut T) -> bool {
        if self.left.is_some() || self.right.is_some() {
            return false;
        };
        self.vert = if (self.dim.width / self.dim.height) as f32 > 1.25 {
            true
        } else if (self.dim.height / self.dim.width) as f32 > 1.25 {
            false
        } else {
            rng.gen::<bool>()
        };
        let max_size = if self.vert {
            self.dim.width - min_size
        } else {
            self.dim.height - min_size
        };

        if max_size <= min_size {
            return false;
        }
        let size = rng.gen_range(min_size, max_size + 1);

        if self.vert {
            self.left = Box::new(Some(Leaf::new(self.dim.x, self.dim.y, size, self.dim.height)));
            self.right = Box::new(Some(Leaf::new(self.dim.x + size, self.dim.y, self.dim.width - size, self.dim.height)));
        } else {
            self.left = Box::new(Some(Leaf::new(self.dim.x, self.dim.y, self.dim.width, size)));
            self.right = Box::new(Some(Leaf::new(self.dim.x, self.dim.y + size, self.dim.width, self.dim.height - size)));
        }

        true
    }
}

fn split<T: Rng>(leaf: &mut Leaf, min_size: i32, rng: &mut T) {
    leaf.split(min_size, rng);
    if let Some(ref mut left) = *leaf.left {
        split(left, min_size, rng);
    }
    if let Some(ref mut right) = *leaf.right {
        split(right, min_size, rng);
    }
}

fn create_rooms<T: Rng>(leaf: &mut Leaf, min_size: i32, rng: &mut T) {
    let mut created_room = false;
    if let Some(ref mut left) = *leaf.left {
        create_rooms(left, min_size, rng);
        created_room = true;
    }
    if let Some(ref mut right) = *leaf.right {
        create_rooms(right, min_size, rng);
        created_room = true;
    }
    if !created_room && leaf.room.is_none() && leaf.dim.width > min_size && leaf.dim.height > min_size {
        let width = rng.gen_range(min_size, leaf.dim.width);
        let height = rng.gen_range(min_size, leaf.dim.height);
        let x = leaf.dim.x + rng.gen_range(0, leaf.dim.width - width);
        let y = leaf.dim.y + rng.gen_range(0, leaf.dim.height - height);
        leaf.room = Some(Rect{x, y, width, height});
    }
}

fn carve<T: Rng>(leaf: &Leaf, map: &mut Map, rng: &mut T) -> Vec<Rect> {
    if let Some(ref room) = leaf.room {
        for x in 0..room.width {
            for y in 0..room.height {
                map.set(x + room.x, y + room.y, 1);
            }
        }
        map.add_room(room.clone());
        return vec![room.clone()]
    } else {
        let mut left_rooms = if let Some(ref left) = *leaf.left {
            carve(left, map, rng)
        } else {
            vec![]
        };
        let mut right_rooms = if let Some(ref right) = *leaf.right {
            carve(right, map, rng)
        } else {
            vec![]
        };

        {
            let left = rng.choose(&left_rooms);
            let right = rng.choose(&right_rooms);
            if let Some(left) = left {
                if let Some(right) = right {
                    carve_tunnel(left.x, left.y, right.x, right.y, map, rng);
                }
            }
        }

        let mut rooms: Vec<Rect> = vec![];
        rooms.append(&mut left_rooms);
        rooms.append(&mut right_rooms);

        return rooms;
    }
}

fn carve_tunnel<T: Rng>(x1: i32, y1: i32, x2: i32, y2: i32, map: &mut Map, rng: &mut T) {
    if rng.gen::<bool>() {
        carve_h_tunnel(x1, y1, x2, map);
        carve_v_tunnel(x2, y2, y1, map);
    } else {
        carve_v_tunnel(x1, y1, y2, map);
        carve_h_tunnel(x2, y2, x1, map);
    }
}

fn carve_h_tunnel(x1: i32, y: i32, x2: i32, map: &mut Map) {
    for x in min(x1, x2)..(max(x1, x2) + 1) {
        map.set(x, y, 1);
    }
}

fn carve_v_tunnel(x: i32, y1: i32, y2: i32, map: &mut Map) {
    for y in min(y1, y2)..(max(y1, y2) + 1) {
        map.set(x, y, 1);
    }
}

pub fn generate<T: Rng>(width: i32, height: i32, min_size: i32, rng: &mut T) -> Map {
    let mut root = Leaf::new(0, 0, width - 2, height - 2);
    split(&mut root, min_size, rng);
    create_rooms(&mut root, min_size, rng);
    let mut map = Map::new(width - 2, height - 2);
    carve(&root, &mut map, rng);
    Map::pad_map(&map)
}

#[cfg(test)]
mod tests {
    use bsp::*;

    fn test_leaf_equality(actual: &Leaf, expected: &Leaf) {
        assert_eq!(actual.dim, expected.dim);
        // assert_eq!(actual.vert, expected.vert);
        assert_eq!(actual.room, expected.room);
        if let Some(ref left) = *actual.left {
            if let Some(ref eleft) = *expected.left {
                test_leaf_equality(left, eleft);
            } else {
                assert!(false);
            }
        } else {
            assert!(expected.left.is_none());
        }
        if let Some(ref right) = *actual.right {
            if let Some(ref eright) = *expected.right {
                test_leaf_equality(right, eright);
            } else {
                assert!(false);
            }
        } else {
            assert!(expected.right.is_none());
        }
    }

    #[test]
    fn test_create_rooms() {
        let mut root = Leaf::new(0, 0, 10, 10);
        let mut rng: XorShiftRng = SeedableRng::from_seed([0, 1, 3, 4]);
        split(&mut root, 3, &mut rng);
        create_rooms(&mut root, 3, &mut rng);
        let leaf = Leaf {
            dim: Rect{x: 0, y: 0, width: 10, height: 10},
            vert: true,
            room: None,
            left: Box::new(Some(Leaf{
                dim: Rect{x: 0, y: 0, width: 10, height: 4},
                vert: true,
                room: None,
                left: Box::new(Some(Leaf{
                    dim: Rect{x: 0, y: 0, width: 6, height: 4},
                    vert: true,
                    room: Some(Rect{x: 0, y: 0, width: 4, height: 3}),
                    left: Box::new(None),
                    right: Box::new(None)
                })),
                right: Box::new(Some(Leaf{
                    dim: Rect{x: 6, y: 0, width: 4, height: 4},
                    vert: true,
                    room: Some(Rect{x: 6, y: 0, width: 3, height: 3}),
                    left: Box::new(None),
                    right: Box::new(None)
                })),
            })),
            right: Box::new(Some(Leaf{
                dim: Rect{x: 0, y: 4, width: 10, height: 6},
                vert: true,
                room: None,
                left: Box::new(Some(Leaf{
                    dim: Rect{x: 0, y: 4, width: 3, height: 6},
                    vert: true,
                    room: None,
                    left: Box::new(None),
                    right: Box::new(None)
                })),
                right: Box::new(Some(Leaf{
                    dim: Rect{x: 3, y: 4, width: 7, height: 6},
                    vert: true,
                    room: None,
                    left: Box::new(Some(Leaf{
                        dim: Rect{x: 3, y: 4, width: 4, height: 6},
                        vert: true,
                        room: Some(Rect{x: 3, y: 4, width: 3, height: 5}),
                        left: Box::new(None),
                        right: Box::new(None)
                    })),
                    right: Box::new(Some(Leaf{
                        dim: Rect{x: 7, y: 4, width: 3, height: 6},
                        vert: true,
                        room: None,
                        left: Box::new(None),
                        right: Box::new(None)
                    })),
                })),
            }))
        };

        test_leaf_equality(&root, &leaf);
    }
}

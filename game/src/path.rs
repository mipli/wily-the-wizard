use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

use crate::spatial::*;
use geo::*;
use crate::map::*;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    priority: i32,
    position: Point,
}


impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        self.priority.cmp(&other.priority).reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn path(start: Point, goal: Point, grid: &SpatialTable, map: &Map) -> Option<Vec<Point>> {
    if start == (-1, -1) || goal == (-1, -1) {
        return None;
    }
    let mut frontier = BinaryHeap::new();
    let mut from: HashMap<Point, Point> = Default::default();
    let mut cost_so_far: HashMap<Point, i32> = Default::default();

    frontier.push(State{priority: 0, position: start});
    from.insert(start, start);
    cost_so_far.insert(start, 0);

    let mut found = false;
    while let Some(State{position, .. }) = frontier.pop() {
        if found || position == goal {
            break;
        }
        let point_cost = match cost_so_far.get(&position) {
            Some(p) => *p,
            None => unreachable!()
        };

        for neighbour in get_neigbours(position.x, position.y, false) {
            if neighbour == goal {
                from.insert(neighbour, position);
                found = true;
                break;
            }
            if can_walk(neighbour, grid, map) {
                let new_cost = point_cost + 1;
                if !cost_so_far.contains_key(&neighbour) || new_cost < cost_so_far[&neighbour] {
                    cost_so_far.insert(neighbour, new_cost);
                    frontier.push(State{priority: heuristic(neighbour, goal), position: neighbour});
                    from.insert(neighbour, position);
                }
            }
        }
    }

    match from.get(&goal) {
        Some(_) => {
            let mut path = vec![goal];
            let mut current = goal;
            while let Some(p) = from.get(&current) {
                if *p == start {
                    break;
                }
                path.push(*p);
                current = *p;
            };
            Some(path)
        },
        None => None
    }
}

fn heuristic(a: Point, b: Point) -> i32 {
    (a.x - b.x).pow(2) + (a.y - b.y).pow(2)
}

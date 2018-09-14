use std::cmp::{Ordering};
use std::collections::BinaryHeap;

use spawning_pool::EntityId;
use crate::components;

#[derive(Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
struct ScheduledEntity {
    entity: EntityId,
    entry_id: i32,
    time: i32
}

impl Ord for ScheduledEntity {
    fn cmp(&self, _other: &ScheduledEntity) -> Ordering {
        // NOTE: I don't know that's the difference between this one
        // and the more explicit fn below. So let's just crash here
        // and see if and when we ever hit this.
        unimplemented!();
    }
}

impl PartialOrd for ScheduledEntity {
    fn partial_cmp(&self, _other: &ScheduledEntity) -> Option<Ordering> {
        // NOTE: I don't know that's the difference between this one
        // and the more explicit fn below. So let's just crash here
        // and see if and when we ever hit this.
        unimplemented!();
    }

    fn lt(&self, other: &ScheduledEntity) -> bool {
        if self.time != other.time {
            self.time > other.time
        } else {
            self.entry_id > other.entry_id
        }
    }

    fn le(&self, other: &ScheduledEntity) -> bool {
        if self.time != other.time {
            self.time >= other.time
        } else {
            self.entry_id > other.entry_id
        }
    }

    fn gt(&self, other: &ScheduledEntity) -> bool {
        if self.time != other.time {
            self.time < other.time
        } else {
            self.entry_id < other.entry_id
        }
    }

    fn ge(&self, other: &ScheduledEntity) -> bool {
        if self.time != other.time {
            self.time <= other.time
        } else {
            self.entry_id < other.entry_id
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Scheduler {
    pub time: i32,
    heap: BinaryHeap<ScheduledEntity>,
    entries: i32,
    pub current: Option<EntityId>
}

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            time: 0,
            entries: 0,
            heap: BinaryHeap::new(),
            current: None
        }
    }

    pub fn schedule_entity(&mut self, entity: EntityId, time: i32, spawning_pool: &components::SpawningPool) {
        if spawning_pool.get::<components::Controller>(entity).is_some() {
            self.heap.push(ScheduledEntity{
                time: self.time + time,
                entry_id: self.entries,
                entity
            });
            self.entries += 1;
        }

        if let Some(current) = self.current {
            if current == entity {
                self.current = None;
            }
        }
    }

    pub fn get_current(&self) -> EntityId {
        self.current.unwrap()
    }

    pub fn tick(&mut self, spawning_pool: &components::SpawningPool) {
        while self.current.is_none() && !self.heap.is_empty() {
            if let Some(se) = self.heap.pop() {
                if spawning_pool.get::<components::Controller>(se.entity).is_some() {
                    self.current = Some(se.entity);
                    self.time = se.time;
                }
            }
        }
    }
}

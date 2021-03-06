use spawning_pool::{EntityId};
use std::collections::HashSet;
use std::cmp::{min, max};

use crate::components;
use geo::*;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SpatialCell {
    pub entities: HashSet<EntityId>,
    pub solid: bool,
    pub solid_count: i32,
    pub opaque: bool,
    pub opaque_count: i32
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SpatialTable {
    pub width: i32,
    pub height: i32,
    pub dirty: bool,
    pub cells: Vec<SpatialCell>,
}


impl SpatialTable {
    pub fn new(width: i32, height: i32) -> Self {
        let size = (width * height) as usize;
        let mut cells = vec![];
        cells.resize(size, SpatialCell::new());
        SpatialTable {
            width,
            height,
            dirty: true,
            cells,
        }
    }

    pub fn reset(&mut self, spawning_pool: &components::SpawningPool) {
        for cell in self.cells.iter_mut() {
            if !cell.entities.is_empty() {
                cell.entities.clear();
                cell.solid = false;
                cell.solid_count = 0;
                cell.opaque = false;
                cell.opaque_count = 0;
            }
        }
        let ids = spawning_pool.get_all::<components::Physics>();
        for (id, physics) in ids {
            let pos = physics.coord;
            self.cells[(pos.x + (pos.y * self.width)) as usize].add_entity(id, spawning_pool);
        }
        self.dirty = true;
    }

    pub fn get<P: Into<Point>>(&self, pos: P) -> Option<&SpatialCell> {
        let p = pos.into();
        self.cells.get((p.x + (p.y * self.width)) as usize)
    }

    pub fn get_mut(&mut self, pos: Point) -> Option<&mut SpatialCell> {
        self.cells.get_mut((pos.x + (pos.y * self.width)) as usize)
    }

    pub fn in_circle<P: Into<Point>>(&self, pos: P, radius: i32) -> Vec<(Point, EntityId)> {
        let pos = pos.into();
        let mut entities = vec![];

        for x in max(0, pos.x - radius)..min(self.width, pos.x + radius + 1) {
            for y in max(0, pos.y - radius)..min(self.height, pos.y + radius + 1) {
                let dist = pos.distance((x, y));
                if dist <= radius as f32 {
                    if let Some(cell) = self.get((x, y)) {
                        for entity in &cell.entities {
                            entities.push(((x, y).into(), *entity));
                        }
                    }
                }
            }
        }
        entities
    }

    pub fn get_closest<P: Into<Point>>(&self, pos: P, range: i32, exclude: bool) -> Option<(Point, EntityId)> {
        let pos = pos.into();
        let required_index = if exclude {
            1 as usize
        } else {
            0 as usize
        };
        let entities = self.get_by_proximity(pos, range);
        if entities.len() <= required_index {
            return None;
        }
        Some(entities[required_index])
    }

    pub fn get_by_proximity<P: Into<Point>>(&self, pos: P, range: i32) -> Vec<(Point, EntityId)> {
        let pos = pos.into();
        let mut entities = self.in_circle(pos, range);
        entities.sort_by(|a, b| {
            let da = pos.tile_distance(a.0);
            let db = pos.tile_distance(b.0);
            da.cmp(&db)
        });
        entities
    }
}

impl SpatialCell {
    pub fn new() -> Self {
        SpatialCell {
            entities: HashSet::new(),
            solid_count: 0,
            solid: false,
            opaque_count: 0,
            opaque: false
        }
    }

    pub fn add_entity(&mut self, entity: EntityId, spawning_pool: &components::SpawningPool) {
        self.entities.insert(entity);

        if let Some(flags) = spawning_pool.get::<components::Flags>(entity) {
            if flags.solid {
                self.solid = true;
                self.solid_count += 1;
            }
            if flags.block_sight {
                self.opaque = true;
                self.opaque_count += 1;
            }
        }
    }

    pub fn remove_entity(&mut self, entity: EntityId, spawning_pool: &mut components::SpawningPool) {
        self.entities.remove(&entity);

        if let Some(flags) = spawning_pool.get::<components::Flags>(entity) {
            if flags.solid {
                self.solid_count -= 1;
                if self.solid_count <= 0 {
                    self.solid = false;
                    self.solid_count = 0;
                }
            }
            if flags.block_sight {
                self.opaque_count -= 1;
                if self.opaque_count <= 0 {
                    self.opaque = true;
                    self.opaque_count = 0;
                }
            }
        }
    }

    pub fn reduce_solid_count(&mut self) {
        self.solid_count -= 1;
        if self.solid_count <= 0 {
            self.solid = false;
            self.solid_count = 0;
        }
    }

    pub fn reduce_opaque_count(&mut self) {
        self.opaque_count -= 1;
        if self.opaque_count <= 0 {
            self.opaque = false;
            self.opaque_count = 0;
        }
    }
}

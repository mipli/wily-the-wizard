extern crate rand;

use rand::Rng;

use spawning_pool::EntityId;

use utils;
use map;

use messages::*;
use geo::*;
use actions::*;
use game::*;
use components;

pub fn confusion(entity: EntityId, state: &mut GameState) -> Option<Action> {
    use components::*;
    let _ = state.spawning_pool.get::<Stats>(entity)?.effects.get(&Effect::Confuse)?;
    let entity_position = get_entity_position(entity, state)?;
    let mut neighbours = get_neigbours(entity_position.x, entity_position.y, false);
    rand::thread_rng().shuffle(&mut neighbours);
    for n in neighbours {
        if map::can_walk(n, &state.spatial_table, &state.map) {
            let (x, y) = entity_position.direction_to(n);
            return Some(Action {
                    actor: Some(entity),
                    target: None,
                    command: Command::WalkDirection{dir: Point::new(x, y)}
                });
        }
    }
    None
}

/*
fn reduce_timer(entity: EntityId, state: &mut GameState) {
    use components::*;
    let name = utils::get_entity_name(entity, &state.spawning_pool);
    if let Some(stats) = state.spawning_pool.get_mut::<Stats>(entity) {
        let remain = match stats.effects.get(&Effect::Confuse) {
            Some(time) => time >= &state.scheduler.time,
            None => false
        };
        if !remain {
            stats.effects.remove(&Effect::Confuse);
            state.messages.log(MessageLevel::Info, format!("The {} is longer confused", name));
        }
    }
}
*/

pub struct DurationSystem {
    last_time: i32
}

impl DurationSystem {
    pub fn new() -> DurationSystem {
        DurationSystem {
            last_time: 0
        }
    }

    pub fn run(&mut self, state: &mut GameState) {
        let time = state.scheduler.time;
        if time < self.last_time + 100 {
            return;
        }
        self.last_time = time;
        self.duration(state);
        self.effects(state);
    }

    fn effects(&mut self, state: &mut GameState) {
        let ids: Vec<EntityId> = state.spawning_pool.get_all::<components::Stats>()
            .iter()
            .map(|(id, _)| *id)
            .collect();
        for id  in ids {
            self.clear_effects(id, state);
        }
    }

    fn clear_effects(&self, entity: EntityId, state: &mut GameState) {
        use components::*;

        let name = utils::get_entity_name(entity, &state.spawning_pool);
        let current_time = state.scheduler.time;
        if let Some(stats) = state.spawning_pool.get_mut::<Stats>(entity) {
            let remove: Vec<_> = stats.effects.iter().filter_map(|(e, t)| {
                println!("Effect time {}, {}", t, current_time);
                if *t < current_time {
                    Some(e.clone())
                } else {
                    None
                }
            }).collect();
            for r in remove {
                state.messages.log(MessageLevel::Info, format!("The {} is longer {}", name, r));
                stats.effects.remove(&r);
            }
        }
    }

    fn duration(&mut self, state: &mut GameState) {
        let ids: Vec<EntityId> = state.spawning_pool.get_all::<components::Duration>()
            .iter()
            .map(|(id, _)| *id)
            .collect();
        for id  in ids {
            self.apply(id, state);
        }
    }

    fn apply(&mut self, id: EntityId, state: &mut GameState) {
        let mut keep = true;
        if let Some(duration) = state.spawning_pool.get_mut::<components::Duration>(id) {
            if duration.spawn_time == 0 {
                duration.spawn_time = self.last_time;
                duration.expire_time = self.last_time + duration.duration;
            } else {
                keep = duration.expire_time > self.last_time;
            }
        }
        if !keep {
            println!("Removing entity");
            state.spawning_pool.remove_entity(id);
        }
    }
}

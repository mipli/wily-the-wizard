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
    let action = get_confusion_action(entity, state);

    if action.is_some() {
        reduce_timer(entity, state);
    }

    action
}

fn get_confusion_action(entity: EntityId, state: &mut GameState) -> Option<Action> {
    let status_effects = state.spawning_pool.get::<components::StatusEffects>(entity)?;
    let _ = status_effects.confused?;
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

fn reduce_timer(entity: EntityId, state: &mut GameState) {
    let name = utils::get_entity_name(entity, &state.spawning_pool);
    if let Some(status_effects) = state.spawning_pool.get_mut::<components::StatusEffects>(entity) {
        if let Some(confused) = status_effects.confused {
            let remain = confused - 1;
            status_effects.confused =  if remain > 0 {
                Some(remain)
            } else {
                state.messages.log(MessageLevel::Info, format!("The {} is longer confused", name));
                None
            };
        }
    }
}

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

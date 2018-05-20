use spawning_pool::{EntityId};
use rand::{thread_rng, Rng};
use map::*;
use geo::*;
use game::*;
use actions::*;
use spells;
use components;
use path;

pub fn melee_attack_entity(actor: EntityId, target: EntityId, state: &mut GameState) -> Option<Vec<Action>> {
    let target_position = get_entity_position(target, state)?;
    let entity_position = get_entity_position(actor, state)?;

    if entity_position.tile_distance(target_position) == 1 {
        Some(vec![Action {
            actor: Some(actor),
            target: Some(ActionTarget::Entity(target)),
            command: Command::AttackEntity{bonus_strength: 0, bonus_defense: 0}
        }])
    } else {
        None
    }
}

pub fn walk_to_position(actor: EntityId, end: Point, state: &mut GameState) -> Option<Vec<Action>> {
    let start= get_entity_position(actor, state)?;
    let next_pos = step_towards_position(actor, start, end, state)?;
    let dir = Point::new(next_pos.x - start.x, next_pos.y - start.y);
    Some(vec![Action {
        actor: Some(actor),
        target: None,
        command: Command::WalkDirection{dir}
    }])
}

pub fn wait_and_forget(actor: EntityId, state: &mut GameState) -> Option<Vec<Action>> {
    use components::*;
    if let Some(mem) = state.spawning_pool.get_mut::<AiMemory>(actor) {
        mem.forget();
    }
    Some(vec![Action {
        actor: Some(actor),
        target: None,
        command: Command::Wait
    }])
}

pub fn cast_spell_at(actor: EntityId, target: EntityId, state: &mut GameState) -> Option<Vec<Action>> {
    use components::*;
    let actor_position = get_entity_position(actor, state)?;
    let target_position = get_entity_position(target, state)?;
    let is_visible = state.spawning_pool.get::<MapMemory>(actor)?.is_visible(target_position.x, target_position.y);
    if !is_visible {
        return None
    }
    let spell = select_spell(actor, &state.spawning_pool)?;
    if actor_position.distance(target_position) < spell.range as f32 {
        return Some(vec![Action {
            actor: Some(actor),
            target: Some(ActionTarget::Entity(target)),
            command: Command::CastSpell{
                spell
            }
        }]);
    } else {
        None
    }
}

fn select_spell(entity: EntityId, spawning_pool: &components::SpawningPool) -> Option<spells::Spell> {
    if let Some(spell_book) = spawning_pool.get::<components::SpellBook>(entity) {
        let spell = thread_rng().choose(&spell_book.spells)?;
        Some(spells::Spell::create(*spell))
    } else {
        None
    }
}

fn step_towards_position(actor: EntityId, start: Point, end: Point, state: &mut GameState) -> Option<Point> {
    use components::*;
    if start == end {
        return None;
    }
    if let Some(mem) = state.spawning_pool.get_mut::<AiMemory>(actor) {
        match mem.remember_path_to(start, end) {
            Some(next) => {
                if can_walk(next, &state.spatial_table, &state.map) {
                    Some(next)
                } else {
                    match path::path(start, end, &state.spatial_table, &state.map) {
                        Some(mut path) => {
                            let next = path.pop();
                            mem.path = Some(path);
                            mem.path_goal = Some(end);
                            next
                        },
                        None => {
                            mem.forget();
                            None
                        }
                    }
                }
            },
            None => {
                match path::path(start, end, &state.spatial_table, &state.map) {
                    Some(mut path) => {
                        let next = path.pop();
                        mem.path = Some(path);
                        mem.path_goal = Some(end);
                        next
                    },
                    None => {
                        mem.forget();
                        None
                    }
                }
            }
        }
    } else {
        None
    }
}

pub fn get_player_position(actor: EntityId, state: &mut GameState) -> Option<Point> {
    use components::*;
    match get_entity_position(state.player, state) {
        Some(pos) => {
            if let Some(map_memory) = state.spawning_pool.get::<MapMemory>(actor) {
                if map_memory.is_visible(pos.x, pos.y) {
                    Some(pos)
                } else {
                    if let Some(mem) = state.spawning_pool.get::<AiMemory>(actor) {
                        mem.path_goal
                    } else {
                        None
                    }
                }
            } else {
                Some(pos)
            }
        }
        None => {
            if let Some(mem) = state.spawning_pool.get_mut::<AiMemory>(actor) {
                mem.forget();
            }
            None
        }
    }
}

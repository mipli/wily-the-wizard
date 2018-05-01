use spawning_pool::{EntityId};
use rand::{thread_rng, Rng};
use geo::*;
use game::*;
use actions::*;
use path;
use spells;
use components;

pub fn perform_basic_ai(actor: EntityId, state: &mut GameState) -> Option<Vec<Action>> {
    use components::*;
    if let Some(entity_position) = get_entity_position(actor, state) {
        let player_position = get_player_position(actor, state);
        if let Some(player_position) = player_position {
            if let Some(mem) = state.spawning_pool.get_mut::<AiMemory>(actor) {
                mem.player_position = Some(player_position);
            }
            match path::path(entity_position, player_position, &state.spatial_table, &state.map) {
                Some(mut path) => {
                    let next = path.pop().unwrap();
                    let dir = Point::new(next.x - entity_position.x, next.y - entity_position.y);
                    return Some(vec![Action {
                            actor: Some(actor),
                            target: None,
                            command: Command::WalkDirection{dir}
                        }]);
                },
                None => {
                    return Some(vec![Action {
                        actor: Some(actor),
                        target: None,
                        command: Command::Wait
                    }]);
                }
            }
        }
    }
    Some(vec![Action {
        actor: Some(actor),
        target: None,
        command: Command::Wait
    }])
}

pub fn perform_spell_ai(actor: EntityId, state: &mut GameState) -> Option<Vec<Action>> {
    use components::*;
    if let Some(entity_position) = get_entity_position(actor, state) {
        let player_position = get_player_position(actor, state);
        if let Some(player_position) = player_position {
            if let Some(mem) = state.spawning_pool.get_mut::<AiMemory>(actor) {
                mem.player_position = Some(player_position);
            }
            let is_visible = match get_entity_position(state.player, state) {
                Some(pos) => {
                    match state.spawning_pool.get::<MapMemory>(actor) {
                        Some(mem) => mem.is_visible(pos.x, pos.y),
                        None => false
                    }
                },
                None => false
            };
            if let Some(spell) = select_spell(actor, &state.spawning_pool) {
                if is_visible && entity_position.distance(player_position) < spell.range as f32 {
                    return Some(vec![Action {
                            actor: Some(actor),
                            target: Some(state.player),
                            command: Command::CastSpell{
                                spell
                            }
                        }]);
                } else {
                    match path::path(entity_position, player_position, &state.spatial_table, &state.map) {
                        Some(mut path) => {
                            let next = path.pop().unwrap();
                            let dir = Point::new(next.x - entity_position.x, next.y - entity_position.y);
                            return Some(vec![Action {
                                    actor: Some(actor),
                                    target: None,
                                    command: Command::WalkDirection{dir}
                                }]);
                        },
                        None => {
                            return Some(vec![Action {
                                actor: Some(actor),
                                target: None,
                                command: Command::Wait
                            }]);
                        }
                    }
                }
            }
        }
    }
    Some(vec![Action {
        actor: Some(actor),
        target: None,
        command: Command::Wait
    }])
}

fn select_spell(entity: EntityId, spawning_pool: &components::SpawningPool) -> Option<spells::Spell> {
    if let Some(spell_book) = spawning_pool.get::<components::SpellBook>(entity) {
        let spell = thread_rng().choose(&spell_book.spells)?;
        Some(spells::Spell::create(*spell))
    } else {
        None
    }
}

fn get_player_position(actor: EntityId, state: &GameState) -> Option<Point> {
    use components::*;
    match get_entity_position(state.player, state) {
        Some(pos) => {
            if let Some(map_memory) = state.spawning_pool.get::<MapMemory>(actor) {
                if map_memory.is_visible(pos.x, pos.y) {
                    Some(pos)
                } else {
                    if let Some(mem) = state.spawning_pool.get::<AiMemory>(actor) {
                        mem.player_position
                    } else {
                        None
                    }
                }
            } else {
                Some(pos)
            }
        }
        None => {
            if let Some(mem) = state.spawning_pool.get::<AiMemory>(actor) {
                mem.player_position
            } else {
                None
            }
        }
    }
}

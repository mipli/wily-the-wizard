use spawning_pool::{EntityId};
use rand::{thread_rng, Rng};
use geo::*;
use game::*;
use actions::*;
use path;
use spells;
use components;

pub fn perform_basic_ai(actor: EntityId, state: &mut GameState) -> Option<Vec<Action>> {
    let player_position = get_player_position(actor, state);
    if let Some(player_position) = player_position {
        match get_dir_to_position(player_position, actor, state) {
            Some(p) if p == Point::new(0, 0) => {
                return Some(vec![Action {
                    actor: Some(actor),
                    target: None,
                    command: Command::Wait
                }]);
            },
            Some(dir) => {
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
        };
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
            if player_position == entity_position {
                if let Some(mem) = state.spawning_pool.get_mut::<AiMemory>(actor) {
                    mem.path_goal = None;
                }
                return Some(vec![Action {
                    actor: Some(actor),
                    target: None,
                    command: Command::Wait
                }]);
            }
            if let Some(mem) = state.spawning_pool.get_mut::<AiMemory>(actor) {
                mem.path_goal = Some(player_position);
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
                            target: Some(ActionTarget::Entity(state.player)),
                            command: Command::CastSpell{
                                spell
                            }
                        }]);
                } else {
                    match get_dir_to_position(player_position, actor, state) {
                        Some(dir) => {
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
                    };
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
            if let Some(mem) = state.spawning_pool.get::<AiMemory>(actor) {
                mem.path_goal
            } else {
                None
            }
        }
    }
}

fn get_dir_to_position(position: Point, actor: EntityId, state: &mut GameState) -> Option<Point> {
    use components::*;

    let entity_position = get_entity_position(actor, state)?;
    if position == entity_position {
        return Some(Point::new(0, 0));
    };
    let next_pos = if let Some(mem) = state.spawning_pool.get_mut::<AiMemory>(actor) {
        match mem.remember_path_to(entity_position, position) {
            Some(next) => Some(next),
            None => {
                match path::path(entity_position, position, &state.spatial_table, &state.map) {
                    Some(mut path) => {
                        let next = path.pop();
                        mem.path = Some(path);
                        mem.path_goal = Some(position);
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
    };
    match next_pos {
        Some(next) => {
            let dir = Point::new(next.x - entity_position.x, next.y - entity_position.y);
            return Some(dir);
        },
        None => None
    }
}

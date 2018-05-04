use geo::*;
use spawning_pool::{EntityId};
use utils;
use actions::*;
use game::*;
use components;
use spells::*;

use rules::definitions::*;

pub fn lightning_strike(action: &mut Action, _state: &GameState, _rejected_actions: &mut Vec<Action>, reaction_actions: &mut Vec<Action>) -> ActionStatus {
    if let  Command::LightningStrike{damage} = action.command {
        reaction_actions.push(Action{
            command: Command::TakeDamage{damage},
            ..*action
        });
    }
    ActionStatus::Accept
}

pub fn validate_spell(action: &mut Action, state: &GameState, _rejected_actions: &mut Vec<Action>, _reaction_actions: &mut Vec<Action>) -> ActionStatus {
    use components::*;
    if let Command::CastSpell{ref spell} = action.command {
        if let Some(actor) = action.actor {
            if state.spawning_pool.get::<Physics>(actor).is_none() {
               return  ActionStatus::Reject;
            }
        }
        if let Some(target) = action.target {
            if state.spawning_pool.get::<Physics>(target).is_none() {
               return  ActionStatus::Reject;
            }
        }
        if let Some(actor) = action.actor {
            if let Some(target) = action.target {
                let actor_position = utils::get_position(actor, &state.spawning_pool).unwrap();
                let target_position = utils::get_position(target, &state.spawning_pool).unwrap();
                let distance = actor_position.distance(target_position);
                if distance > spell.range as f32 {
                    return  ActionStatus::Reject;
                }
            }
        }
    }
    ActionStatus::Accept
}

pub fn cast_spell(action: &mut Action, state: &GameState, _rejected_actions: &mut Vec<Action>, reaction_actions: &mut Vec<Action>) -> ActionStatus {
    match action.command {
        Command::CastSpell{ref spell} => {
            if cast(spell, action.actor, action.target, state, reaction_actions) {
                ActionStatus::Accept
            } else {
                ActionStatus::Reject
            }
        },
        _ => ActionStatus::Accept
    }
}

enum SpellTarget {
    Entity(EntityId),
    Position(Point)
}

fn cast(spell: &Spell, caster: Option<EntityId>, target: Option<EntityId>, state: &GameState, reaction_actions: &mut Vec<Action>) -> bool {
    let spell_target = match spell.target {
        SpellTargetType::Entity => Some(SpellTarget::Entity(target.unwrap())),
        SpellTargetType::Closest => {
            let target = get_closest_target(caster.unwrap(), state);
            if target.is_some() {
                Some(SpellTarget::Entity(target.unwrap()))
            } else {
                None
            }
        },
        SpellTargetType::Spot => {
            if let Some(id) = caster {
                if let Some(pos) = utils::get_position(id, &state.spawning_pool) {
                    Some(SpellTarget::Position(pos))
                } else {
                    None
                }
            } else {
                None
            }
        }
    };
    if spell_target.is_none() {
        return false;
    }
    match spell.kind {
        Spells::Fog => {
            let target = match spell_target {
                Some(SpellTarget::Position(pos)) => pos,
                _ => return false
            };
            reaction_actions.push(Action{
                actor: caster,
                target: None,
                command: Command::SpawnFog{pos: target}
            });
        },
        Spells::MagicMissile => {
            let target = match spell_target {
                Some(SpellTarget::Entity(id)) => id,
                _ => return false
            };
            reaction_actions.push(Action{
                actor: caster,
                target: Some(target),
                command: Command::TakeDamage{damage: spell.power}
            });
        },
        Spells::LightningStrike => {
            let target = match spell_target {
                Some(SpellTarget::Entity(id)) => id,
                _ => return false
            };
            reaction_actions.push(Action{
                actor: caster,
                target: Some(target),
                command: Command::LightningStrike{damage: spell.power}
            });
        },
        Spells::Confusion => {
            let target = match spell_target {
                Some(SpellTarget::Entity(id)) => id,
                _ => return false
            };
            reaction_actions.push(Action{
                actor: caster,
                target: Some(target),
                command: Command::Confuse
            });
        },
        Spells::Heal => {
            let target = match spell_target {
                Some(SpellTarget::Entity(id)) => id,
                _ => return false
            };
            reaction_actions.push(Action{
                actor: caster,
                target: Some(target),
                command: Command::Heal{amount: spell.power}
            });
        }
    }
    return true;
}

fn get_closest_target(caster: EntityId, state: &GameState) -> Option<EntityId> {
    let pos = match state.spawning_pool.get::<components::Physics>(caster) {
        Some(physics) => Some(physics.coord),
        None => None
    };
    if let Some(pos) = pos {
        let circles: Vec<(Point, EntityId)> = state.spatial_table.in_circle(pos, 5);
        let mut entities: Vec<(EntityId, i32)> = circles.iter().map(|&(_, id)| {
            let physics = state.spawning_pool.get::<components::Physics>(id).unwrap();
            let delta = pos.distance(physics.coord) as i32;
            (id, delta)
        }).filter(|&(ref id, _)| {
            let stats = state.spawning_pool.get::<components::Stats>(*id);
            stats.is_some()
        }).collect();
        if entities.len() > 1 {
            entities.sort_by(|a, b| {
                a.1.cmp(&b.1)
            });
           return Some(entities[1].0);
        }
    }
    None
}

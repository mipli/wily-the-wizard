use inflector::Inflector;

use geo::*;
use std::cmp::min;
use tcod::colors;
use game::*;
use components;
use components::{SpawningPool};

mod items;
mod definitions;

use utils;
use messages::*;
pub use self::definitions::*;
use self::items::*;

#[derive(Debug, PartialEq, Eq)]
pub enum ActionResult {
    Failed,
    Performed{time: i32}
}

pub fn perform_action(action: &Action, game_state: &mut GameState) -> ActionResult {
    match action.command {
        Command::Wait => ActionResult::Performed{time: 100},
        Command::Abort => ActionResult::Failed,
        Command::DescendStairs => {
            game_state.new_level();
            ActionResult::Performed{time: 100}
        },
        Command::AttackEntity{..} => {
            ActionResult::Performed{time: 100}
        },
        Command::WalkDirection{dir} => {
            if let Some(mut physics) = game_state.spawning_pool.get_mut::<components::Physics>(action.actor.unwrap()) {
                physics.coord += dir;
            }
            ActionResult::Performed{time: 100}
        },
        Command::SpawnFog{..} => {
            perform_spawn_fog(action, game_state);
            ActionResult::Performed{time: 0}
        },
        Command::TakeDamage{..} => {
            perform_take_damage(action, game_state);
            ActionResult::Performed{time: 0}
        },
        Command::OpenDoor{..} => {
            perform_open_door(action, game_state);
            ActionResult::Performed{time: 100}
        },
        Command::KillEntity => {
            perform_kill_entity(action, game_state);
            ActionResult::Performed{time: 0}
        },
        Command::PickUpItem{..} => {
            perform_pick_up_item(action, game_state);
            ActionResult::Performed{time: 100}
        },
        Command::UseItem{..} => {
            perform_use_item(action, game_state);
            ActionResult::Performed{time: 0}
        },
        Command::EquipItem{..} => {
            perform_equip_item(action, game_state);
            ActionResult::Performed{time: 50}
        },
        Command::UnequipItem{..} => {
            perform_unequip_item(action, game_state);
            ActionResult::Performed{time: 0}
        },
        Command::DropItem{..} => {
            perform_drop_item(action, game_state);
            ActionResult::Performed{time: 0}
        },
        Command::Heal{..} => {
            perform_heal(action, game_state);
            ActionResult::Performed{time: 0}
        },
        Command::Confuse => {
            if perform_confuse(action, game_state) {
                ActionResult::Performed{time: 100}
            } else {
                ActionResult::Failed
            }
        },
        Command::CastSpell{ref spell} => {
            let actor = utils::get_actor_name(action, &game_state.spawning_pool);
            let msg = match action.target {
                Some(_) => {
                    let target = utils::get_target_name(action, &game_state.spawning_pool);
                    format!("The {} is casting {} on {} ", actor, spell.name, target)
                },
                None => format!("The {} is casting {}", actor, spell.name)
            };
            game_state.messages.log(MessageLevel::Spell, msg);
            ActionResult::Performed{time: 100}
        },
        Command::DestroyItem{..} => {
            perform_destroy_item(action, game_state);
            ActionResult::Performed{time: 0}
        },
        _ => {
            ActionResult::Performed{time: 0}
        }
    }
}
fn perform_spawn_fog(action: &Action, state: &mut GameState) {
    let coords = if let Command::SpawnFog{pos} = action.command {
        let mut coords: Vec<Point> = get_neigbours(pos.x, pos.y, false).into_iter().filter(|&pos| {
            state.map.is_floor(pos)
        }).collect();
        if state.map.is_floor(pos) {
            coords.push(pos);
        }
        coords
    } else {
        vec![]
    };
    for coord in coords {
        create_fog_at(coord, &mut state.spawning_pool);
    }
}

fn create_fog_at(pos: Point, spawning_pool: &mut SpawningPool) {
    let fog = spawning_pool.spawn_entity();
    spawning_pool.set(fog, components::Visual{
        always_display: false,
        glyph: '#',
        color: colors::LIGHTEST_SKY
    });
    spawning_pool.set(fog, components::Physics{
        coord: pos,
    });
    spawning_pool.set(fog, components::Duration{
        spawn_time: 0,
        expire_time: 0,
        duration: 1000
    });
    spawning_pool.set(fog, components::Flags{
        block_sight: true, 
        solid: false
    });
    spawning_pool.set(fog, components::Information{
        name: "Fog".to_string()
    });
}

fn perform_confuse(action: &Action, game_state: &mut GameState) -> bool {
    let performed = if let Some(target) = action.target {
        if let Some(status_effects) = game_state.spawning_pool.get_mut::<components::StatusEffects>(target) {
            status_effects.confused = Some(5);
            return true;
        };
        game_state.spawning_pool.set(target, components::StatusEffects{
            confused: Some(5)
        });
        true
    } else {
        false
    };

    if performed {
        let name = utils::get_target_name(action, &game_state.spawning_pool);
        if action.actor.unwrap() == game_state.player {
            game_state.messages.log(MessageLevel::Important, format!("The {} is confused!", name));
        } else {
            game_state.messages.log(MessageLevel::Info, format!("The {} is confused!", name));
        };
    }
    performed
}

fn perform_heal(action: &Action, game_state: &mut GameState) {
    let amount = match action.command {
        Command::Heal{amount} => amount,
        _ => 0
    };
    let mut performed = false;
    if let Some(stats) = game_state.spawning_pool.get_mut::<components::Stats>(action.target.unwrap()) {
        stats.health = min(stats.health + amount, stats.max_health);
        performed = true;
    }
    if performed {
        let actor_name = utils::get_actor_name(action, &game_state.spawning_pool);
        let target_name = utils::get_target_name(action, &game_state.spawning_pool);
        if actor_name == target_name {
            game_state.messages.log(MessageLevel::Info, format!("{} healed for {}", target_name.to_sentence_case(), amount));
        } else {
            game_state.messages.log(MessageLevel::Info, format!("{} healed {} for {}", actor_name.to_sentence_case(), target_name, amount));
        }
    }
}

fn perform_drop_item(action: &Action, game_state: &mut GameState) {
    let item_id = match action.command {
        Command::DropItem{item_id} => Some(item_id),
        _ => None
    };
    if let Some(inventory) = game_state.spawning_pool.get_mut::<components::Inventory>(action.actor.unwrap()) {
        inventory.items.retain(|i| i != &item_id.unwrap());
    }
    let pos = match game_state.spawning_pool.get::<components::Physics>(action.actor.unwrap()) {
        Some(physics) => Some(physics.coord),
        None => None
    };
    if let Some(pos) = pos {
        game_state.spawning_pool.set(item_id.unwrap(), components::Physics{coord: pos});
    }
}

fn perform_equip_item(action: &Action, game_state: &mut GameState) {
    if let Command::EquipItem{item_id} = action.command {
        let mut performed = false;
        let slot = match game_state.spawning_pool.get::<components::Item>(item_id) {
            Some(item) => item.equip,
            None => None
        };
        if let Some(mut equipment) = game_state.spawning_pool.get_mut::<components::Equipment>(action.actor.unwrap()) {
            if let Some(slot) = slot {
                performed = true;
                equipment.items.insert(slot, item_id);
            }
        }
        if performed {
            let name = utils::get_actor_name(action, &game_state.spawning_pool);
            let item_name = utils::get_entity_name(item_id, &game_state.spawning_pool);
            game_state.messages.log(MessageLevel::Info, format!("{} equips {}", name.to_sentence_case(), item_name));
        }
    }
}

fn perform_unequip_item(action: &Action, game_state: &mut GameState) {
    if let Command::UnequipItem{item_id} = action.command {
        let mut performed = false;
        let slot = match game_state.spawning_pool.get::<components::Item>(item_id) {
            Some(item) => item.equip,
            None => None
        };
        if let Some(mut equipment) = game_state.spawning_pool.get_mut::<components::Equipment>(action.actor.unwrap()) {
            if let Some(slot) = slot {
                performed = true;
                equipment.items.remove(&slot);
            }
        }
        if performed {
            let name = utils::get_actor_name(action, &game_state.spawning_pool);
            let item_name = utils::get_entity_name(item_id, &game_state.spawning_pool);
            game_state.messages.log(MessageLevel::Info, format!("{} takes off the {}", name.to_sentence_case(), item_name));
        }
    }
}

fn perform_destroy_item(action: &Action, game_state: &mut GameState) {
    if let Command::DestroyItem{item_id} = action.command {
        game_state.spawning_pool.remove_entity(item_id);
    }
}

fn perform_pick_up_item(action: &Action, game_state: &mut GameState) {
    if let Command::PickUpItem{item_id} = action.command {
        let mut picked = false;
        if let Some(mut inventory) = game_state.spawning_pool.get_mut::<components::Inventory>(action.actor.unwrap()) {
            inventory.items.push(item_id);
            picked = true;
        }
        if picked {
            game_state.spawning_pool.remove::<components::Physics>(item_id);
            let name = utils::get_actor_name(action, &game_state.spawning_pool);
            let item_name = utils::get_entity_name(item_id, &game_state.spawning_pool);
            game_state.messages.log(MessageLevel::Info, format!("{} picked up {}", name.to_sentence_case(), item_name));
        }
    }

}

fn perform_kill_entity(action: &Action, game_state: &mut GameState) {
    let name = utils::get_actor_name(action, &game_state.spawning_pool);
    if action.actor.unwrap() == game_state.player {
        game_state.messages.log(MessageLevel::Important, format!("The {} has died!", name));
    } else {
        game_state.messages.log(MessageLevel::Info, format!("The {} has died!", name));
    }
    game_state.spawning_pool.remove_entity(action.actor.unwrap());
}

fn perform_take_damage(action: &Action, game_state: &mut GameState) {
    let damage = match action.command {
        Command::TakeDamage{damage} => {
            damage
        },
        _ => unreachable!()
    };

    if let Some(target) = action.target {
        if let Some(mut stats) = game_state.spawning_pool.get_mut::<components::Stats>(target) {
            stats.health -= damage;
        }
    }

    let attacker_name = utils::get_actor_name(action, &game_state.spawning_pool);
    let target_name = utils::get_target_name(action, &game_state.spawning_pool);

    if action.target.unwrap() == game_state.player {
        game_state.messages.log(MessageLevel::Important, format!("The {} attacked the {} for {}", attacker_name, target_name, damage));
    } else {
        game_state.messages.log(MessageLevel::Info, format!("The {} attacked the {} for {}", attacker_name, target_name, damage));
    };
}

fn perform_open_door(action: &Action, game_state: &mut GameState) {
    let id = match action.command {
        Command::OpenDoor{entity} => entity,
        _ => unreachable!()
    };

    game_state.spawning_pool.set(id, components::Visual{always_display: true, glyph: '-', color: colors::WHITE});
    game_state.spawning_pool.set(id, components::Information{name: "open door".to_string()});
    game_state.spawning_pool.set(id, components::Door{opened: true});
    let flags = game_state.spawning_pool.get_mut::<components::Flags>(id).unwrap();
    flags.solid = false;
    flags.block_sight = false;
}

/*
fn perform_attack_entity(action: &Action, game_state: &GameState, reaction_actions: &mut Vec<Action>) {
    if let Command::AttackEntity{bonus_strength, bonus_defense} = action.command {
        if let Some(actor) = action.actor {
            let target_id = match action.target {
                Some(id) => id,
                _ => unreachable!()
            };

            let strength = match game_state.spawning_pool.get::<components::Stats>(actor) {
                Some(stats) => stats.strength,
                None => 0 
            };
            let target_defense = bonus_defense + match game_state.spawning_pool.get::<components::Stats>(target_id) {
                Some(stats) => stats.defense,
                None => 0 
            };
            let attack_strength = bonus_strength + strength;

            let damage = max(1, attack_strength - target_defense);

            reaction_actions.push(Action{
                target: Some(target_id),
                actor: action.actor,
                command: Command::TakeDamage{damage}
            });
        }
    }
}
*/

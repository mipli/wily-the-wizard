use inflector::Inflector;

use std::cmp::min;
use tcod::colors;
use game::*;
use components;

mod items;
mod definitions;

use utils;
use messages::*;
pub use self::definitions::*;
use self::items::*;
use spells;

pub fn perform_action(action: &Action, game_state: &mut GameState, reactions_actions: &mut Vec<Action>) -> bool {
    game_state.spatial_table.update(action, &mut game_state.spawning_pool);
    match action.command {
        Command::Wait | Command::PreKillEntity => true,
        Command::Abort => false,
        Command::DescendStairs => {
            game_state.new_level();
            true
        },
        Command::WalkDirection{dir} => {
            if let Some(mut physics) = game_state.spawning_pool.get_mut::<components::Physics>(action.actor.unwrap()) {
                physics.coord += dir;
            }
            true
        },
        Command::TakeDamage{..} => {
            perform_take_damage(action, game_state, reactions_actions);
            true
        },
        Command::OpenDoor{..} => {
            perform_open_door(action, game_state, reactions_actions);
            true
        },
        Command::KillEntity => {
            perform_kill_entity(action, game_state, reactions_actions);
            true
        },
        Command::PickUpItem{..} => {
            perform_pick_up_item(action, game_state, reactions_actions);
            true
        },
        Command::UseItem{..} => {
            perform_use_item(action, game_state, reactions_actions)
        },
        Command::EquipItem{..} => {
            perform_equip_item(action, game_state, reactions_actions);
            true
        },
        Command::UnequipItem{..} => {
            perform_unequip_item(action, game_state, reactions_actions);
            true
        },
        Command::DropItem{..} => {
            perform_drop_item(action, game_state, reactions_actions);
            false
        },
        Command::Heal{..} => {
            perform_heal(action, game_state, reactions_actions);
            true
        },
        Command::LightningStrike{..} => {
            perform_lightning_strike(action, game_state, reactions_actions);
            true
        },
        Command::Confuse => {
            perform_confuse(action, game_state, reactions_actions)
        },
        Command::CastSpell{..} => {
            perform_cast_spell(action, game_state, reactions_actions);
            true
        },
        Command::DestroyItem{..} => {
            perform_destroy_item(action, game_state, reactions_actions);
            false
        },
        _ => {
            false
        }
    }
}

fn perform_cast_spell(action: &Action, state: &mut GameState, reaction_actions: &mut Vec<Action>) {
    if let Command::CastSpell{ref spell} = action.command {
        println!("Casting {}: {:?} > {:?}", spell.name, action.actor, action.target);
        spells::cast(spell, action.actor, action.target, state, reaction_actions);
    }
}

fn perform_confuse(action: &Action, game_state: &mut GameState, _reaction_actions: &mut Vec<Action>) -> bool {
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

fn perform_lightning_strike(action: &Action, _game_state: &mut GameState, reaction_actions: &mut Vec<Action>) {
    reaction_actions.push(Action{
        command: Command::TakeDamage{damage: 10},
        ..*action
    });
}

fn perform_heal(action: &Action, game_state: &mut GameState, _reactions_actions: &mut Vec<Action>) {
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

fn perform_drop_item(action: &Action, game_state: &mut GameState, _reactions_actions: &mut Vec<Action>) {
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

fn perform_equip_item(action: &Action, game_state: &mut GameState, _reactions_actions: &mut Vec<Action>) {
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

fn perform_unequip_item(action: &Action, game_state: &mut GameState, _reactions_actions: &mut Vec<Action>) {
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

fn perform_destroy_item(action: &Action, game_state: &mut GameState, _reactions_actions: &mut Vec<Action>) {
    if let Command::DestroyItem{item_id} = action.command {
        game_state.spawning_pool.remove_entity(item_id);
    }
}

fn perform_pick_up_item(action: &Action, game_state: &mut GameState, _reactions_actions: &mut Vec<Action>) {
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

fn perform_kill_entity(action: &Action, game_state: &mut GameState, _reactions_actions: &mut Vec<Action>) {
    let name = utils::get_actor_name(action, &game_state.spawning_pool);
    if action.actor.unwrap() == game_state.player {
        game_state.messages.log(MessageLevel::Important, format!("The {} has died!", name));
    } else {
        game_state.messages.log(MessageLevel::Info, format!("The {} has died!", name));
    }
    game_state.spawning_pool.remove_entity(action.actor.unwrap());
}

fn perform_take_damage(action: &Action, game_state: &mut GameState, reactions_actions: &mut Vec<Action>) {
    let damage = match action.command {
        Command::TakeDamage{damage} => {
            damage
        },
        _ => unreachable!()
    };

    let mut performed = false;

    if let Some(target) = action.target {
        if let Some(mut stats) = game_state.spawning_pool.get_mut::<components::Stats>(target) {
            stats.health -= damage;

            if stats.health <= 0 {
                reactions_actions.push(Action{
                    actor: Some(target),
                    target: None,
                    command: Command::KillEntity
                });
            }

            performed = true;
        }
    }

    if performed {
        let attacker_name = utils::get_actor_name(action, &game_state.spawning_pool);
        let target_name = utils::get_target_name(action, &game_state.spawning_pool);

        if action.target.unwrap() == game_state.player {
            game_state.messages.log(MessageLevel::Important, format!("The {} attacked the {} for {}", attacker_name, target_name, damage));
        } else {
            game_state.messages.log(MessageLevel::Info, format!("The {} attacked the {} for {}", attacker_name, target_name, damage));
        };
    }
}

fn perform_open_door(action: &Action, game_state: &mut GameState, _reactions_actions: &mut Vec<Action>) {
    let id = match action.command {
        Command::OpenDoor{entity} => entity,
        _ => unreachable!()
    };

    game_state.spawning_pool.set(id, components::Visual{glyph: '-', color: colors::WHITE});
    game_state.spawning_pool.set(id, components::Information{name: "open door".to_string()});
    game_state.spawning_pool.set(id, components::Door{opened: true});
    let flags = game_state.spawning_pool.get_mut::<components::Flags>(id).unwrap();
    flags.solid = false;
    flags.block_sight = false;
}

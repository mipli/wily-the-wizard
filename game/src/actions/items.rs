use inflector::Inflector;

use spawning_pool::{EntityId};
use actions::definitions::*;
use utils;
use messages::*;
use game::*;
use components;
use spells;

pub fn perform_use_item(action: &Action, game_state: &mut GameState, reaction_actions: &mut Vec<Action>) {
    if let Some(on_use) = get_callback(action, &game_state.spawning_pool) {
        if let Command::UseItem{item_id} = action.command {
            if let Some(item) = game_state.spawning_pool.get::<components::Item>(item_id) {
                match on_use {
                    components::OnUseCallback::Spell(spell) => {
                        destroy_item(item_id, action, reaction_actions);
                        let mut act = Action {
                            actor: action.actor,
                            target: None,
                            command: Command::CastSpell{spell: spells::Spell::create(spell)}
                        };
                        if item.kind == components::ItemKind::Potion {
                            act.target = action.actor;
                        }
                        reaction_actions.push(act);
                    }
                }
                let name = utils::get_actor_name(action, &game_state.spawning_pool);
                let item_name = utils::get_entity_name(item_id, &game_state.spawning_pool);
                game_state.messages.log(MessageLevel::Info, format!("{} uses {}", name.to_sentence_case(), item_name));
            }
        }
    }
}

fn get_callback(action: &Action, spawning_pool: &components::SpawningPool) -> Option<components::OnUseCallback> {
    let item_id = match action.command {
        Command::UseItem{item_id} => Some(item_id),
        _ => None
    };
    let item = match item_id {
        Some(item_id) => {
            match spawning_pool.get::<components::Item>(item_id) {
                Some(i) => Some(i),
                None => None
            }
        },
        None => None
    };
    if let Some(item) = item {
        return item.on_use.clone();
    }
    None
}

fn destroy_item(item_id: EntityId, action: &Action, reaction_actions: &mut Vec<Action>) {
    reaction_actions.push(Action{
        actor: action.actor,
        target: None,
        command: Command::DestroyItem{item_id}
    });
}
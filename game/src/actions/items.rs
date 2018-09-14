use inflector::Inflector;

use crate::actions::definitions::*;
use crate::utils;
use crate::messages::*;
use crate::game::*;

pub fn perform_use_item(action: &Action, game_state: &mut GameState) {
    if let Command::UseItem{item_id} = action.command {
        let name = utils::get_actor_name(action, &game_state.spawning_pool);
        let item_name = utils::get_entity_name(item_id, &game_state.spawning_pool);
        game_state.messages.log(MessageLevel::Info, format!("{} uses {}", name.to_sentence_case(), item_name));
    }
}

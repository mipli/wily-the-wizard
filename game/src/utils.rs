use components;
use spawning_pool::EntityId;
use actions::{ActionTarget, Action};
use geo::*;


pub fn get_entity_name(id: EntityId, spawning_pool: &components::SpawningPool) -> String {
    match spawning_pool.get::<components::Information>(id) {
        Some(info) => info.name.clone(),
        None => "Unknown".to_string()
    }
}

pub fn get_actor_name(action: &Action, spawning_pool: &components::SpawningPool) -> String {
    match action.actor {
        Some(actor) => get_entity_name(actor, spawning_pool),
        None => "Unknown".to_string()
    }
}

pub fn get_target_name(action: &Action, spawning_pool: &components::SpawningPool) -> String {
    match action.target {
        Some(ActionTarget::Entity(actor)) => get_entity_name(actor, spawning_pool),
        _ => "Unknown".to_string()
    }
}

pub fn get_position(entity: EntityId, spawning_pool: &components::SpawningPool) -> Option<Point> {
    let physics = spawning_pool.get::<components::Physics>(entity)?;
    Some(physics.coord)
}

pub fn get_glyph(entity: EntityId, spawning_pool: &components::SpawningPool) -> Option<char> {
    let visual = spawning_pool.get::<components::Visual>(entity)?;
    Some(visual.glyph)
}

pub fn get_strength_bonus(entity: EntityId, spawning_pool: &components::SpawningPool) -> i32 {
    let mut strength = 0;
    if let Some(equipment) = spawning_pool.get::<components::Equipment>(entity) {
        for item_id in equipment.items.values() {
            if let Some(item) = spawning_pool.get::<components::Item>(*item_id) {
                if let Some(ref stats) = item.statistics_bonus {
                    strength += stats.strength;
                }
            }
        }
    }
    strength
}

pub fn get_defense_bonus(entity: EntityId, spawning_pool: &components::SpawningPool) -> i32 {
    let mut defense = 0;
    if let Some(equipment) = spawning_pool.get::<components::Equipment>(entity) {
        for item_id in equipment.items.values() {
            if let Some(item) = spawning_pool.get::<components::Item>(*item_id) {
                if let Some(ref stats) = item.statistics_bonus {
                    defense += stats.defense;
                }
            }
        }
    }
     defense
}

pub fn describe_entity(entity: EntityId, spawning_pool: &components::SpawningPool) -> String {
    let name = get_entity_name(entity, spawning_pool);
    let mut wielding = "".to_string();
    if let Some(equipment) = spawning_pool.get::<components::Equipment>(entity) {
        if let Some(item_id) = equipment.items.get(&components::EquipmentSlot::RightHand) {
            wielding = get_entity_name(*item_id, spawning_pool);
        }
    }
    let glyph = match spawning_pool.get::<components::Visual>(entity) {
        Some(visual) => visual.glyph,
        None => ' '
    };
    if wielding != "" {
        return format!("({}) A {} wielding a {}", glyph, name, wielding);
    } else {
        return format!("({}) A {}", glyph, name);
    }
}

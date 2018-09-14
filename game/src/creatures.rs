use std::io::Read;
use std::fs::File;
use tcod::colors;

use spawning_pool::{EntityId};
use geo::*;
use crate::components;
use crate::spells;

#[derive(Debug)]
pub struct CreatureData {
    pub name: String,
    pub glyph: char,
    pub color: colors::Color,
    pub health: i32,
    pub strength: i32,
    pub defense: i32,
    pub ai: components::AI
}

pub fn load_creatures() -> Vec<CreatureData> {
    use yaml_rust::YamlLoader;
    let mut data = String::new();
    let mut creatures = vec![];
    if let Ok(mut file) = File::open("./data/creatures.yaml") {
        let _ = file.read_to_string(&mut data);
        let docs = YamlLoader::load_from_str(&data).unwrap();
        for base_docs in docs[0]["Base"].as_vec().unwrap() {
            let base = base_docs.as_hash().unwrap();
            for (_, data) in base.iter() {
                let ai = match data["ai"].as_str().unwrap() {
                    "basic" => components::AI::Basic,
                    "spell caster" => components::AI::SpellCaster,
                    _ => unreachable!()
                };
                creatures.push(CreatureData{
                    name: data["name"].as_str().unwrap().to_string(),
                    glyph: data["glyph"].as_str().unwrap().chars().next().unwrap(),
                    color: get_color(data["color"].as_str().unwrap()),
                    health: data["health"].as_i64().unwrap() as i32,
                    strength: data["strength"].as_i64().unwrap() as i32,
                    defense: data["defense"].as_i64().unwrap() as i32,
                    ai
                });
            }
        }
    }
    creatures
}

fn get_color(name: &str) -> colors::Color {
    match name {
        "light green" => colors::LIGHT_GREEN,
        _ => colors::PINK
    }
}

pub fn create_creature(data: &CreatureData, pos: Point, width: i32, height: i32, spawning_pool: &mut components::SpawningPool) -> EntityId {
    let creature = spawning_pool.spawn_entity();
    spawning_pool.set(creature, components::Visual{
        always_display: false,
        glyph: data.glyph, 
        color: data.color
    });
    spawning_pool.set(creature, components::Physics{
        coord: pos,
    });
    spawning_pool.set(creature, components::Flags{
        block_sight: false, 
        solid: true
    });
    spawning_pool.set(creature, components::Controller{
        ai: data.ai
    });
    spawning_pool.set(creature, components::Information{
        faction: components::Faction::Enemy,
        name: data.name.to_string()
    });
    spawning_pool.set(creature, components::Stats::new(
        data.health,
        data.strength,
        data.defense
    ));
    spawning_pool.set(creature, components::MapMemory::new(width, height));
    spawning_pool.set(creature, components::AiMemory::new());
    if data.ai == components::AI::SpellCaster {
        spawning_pool.set(creature, components::SpellBook{
            spells: vec![spells::Spells::MagicMissile]
        });
    }
    creature
}

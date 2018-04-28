use std::io::Read;
use std::fs::File;
use tcod::colors;

use spawning_pool::{EntityId};
use components;
use point::*;

#[derive(Debug)]
pub struct CreatureData {
    pub name: String,
    pub glyph: char,
    pub color: colors::Color,
    pub health: i32,
    pub strength: i32,
    pub defense: i32
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
                creatures.push(CreatureData{
                    name: data["name"].as_str().unwrap().to_string(),
                    glyph: data["glyph"].as_str().unwrap().chars().next().unwrap(),
                    color: get_color(data["color"].as_str().unwrap()),
                    health: data["health"].as_i64().unwrap() as i32,
                    strength: data["strength"].as_i64().unwrap() as i32,
                    defense: data["defense"].as_i64().unwrap() as i32,
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

pub fn create_creature(data: &CreatureData, pos: Point, spawning_pool: &mut components::SpawningPool) -> EntityId {
    let creature = spawning_pool.spawn_entity();
    spawning_pool.set(creature, components::Visual{
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
        ai: components::AI::Basic
    });
    spawning_pool.set(creature, components::Information{
        name: data.name.to_string()
    });
    spawning_pool.set(creature, components::Stats{
        faction: components::Faction::Enemy,
        max_health: data.health,
        health: data.health,
        strength: data.strength,
        defense: data.defense
    });
    creature
}
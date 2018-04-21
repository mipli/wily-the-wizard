use tcod;
use tcod::colors;

use rand;
use rand::*;
use rand::distributions::{IndependentSample, Weighted, WeightedChoice};

use spells;
use spatial::*;
use generator;
use components;
use point::*;
use spawning_pool::{EntityId};
use creatures::*;
use scheduler::{Scheduler};

#[derive(Copy, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum TileType {
    Wall,
    Floor
}

#[derive(Serialize, Deserialize)]
pub struct Cell {
    pub tile_type: TileType,
    pub blocks_movement: bool,
    pub blocks_sight: bool
}

impl Cell {
    pub fn get_render_info(&self, visible: bool) -> (char, tcod::colors::Color, tcod::colors::Color) {
        let color = if visible {
            colors::DARK_GREY
        } else {
            colors::DARKEST_GREY
        };
        match self.tile_type {
            TileType::Wall => ('#', color, tcod::colors::Color{r: 0, g: 20, b: 35}),
            TileType::Floor => ('.', color, tcod::colors::Color{r: 0, g: 10, b: 20})
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Map {
    pub dimensions: Point,
    pub cells: Vec<Cell>,
}

impl Map {
    pub fn new(data: &[Vec<i32>]) -> Map {
        let dimensions = (data.len(), data[0].len());
        let mut cells = vec![];

        for y in 0..(dimensions.1) {
            for x in data {
                let tile_type = match x[y] {
                    0 => TileType::Wall,
                    _ => TileType::Floor
                };
                cells.push(Cell{
                    tile_type,
                    blocks_movement: tile_type == TileType::Wall,
                    blocks_sight: tile_type == TileType::Wall
                });
            }
        }

        Map {
            dimensions: dimensions.into(),
            cells
        }
    }

    pub fn get_cell(&self, x: i32, y: i32) -> &Cell {
        assert!(x > -1 && x < self.dimensions.x);
        assert!(y > -1 && y < self.dimensions.y);
        &self.cells[(x + y * self.dimensions.x) as usize]
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.dimensions.x && y >= 0 && y < self.dimensions.y
    }

    pub fn get_neigbours(&self, x: i32, y: i32) -> Vec<(Point, &Cell)> {
        let mut cells: Vec<(Point, &Cell)> = vec![];

        for dir in &[(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)] {
            let p: Point = (x + dir.0, y + dir.1).into();
            if self.in_bounds(p.x, p.y) {
                cells.push((p, self.get_cell(p.x, p.y)));
            }
        }
        cells
    }
}

pub fn can_walk(position: Point, grid: &SpatialTable, map: &Map) -> bool {
    match grid.get(position) {
        Some(cell) => {
            !cell.solid && map.get_cell(position.x, position.y).tile_type == TileType::Floor
        },
        None => {
            false
        }
    }
}

pub fn create_map(player: EntityId, width: i32, height: i32, spawning_pool: &mut components::SpawningPool, scheduler: &mut Scheduler) -> Map{
    scheduler.add_entity(player, spawning_pool);
    let creatures = load_creatures();

    let (cells, rooms) = generator::generate_map(width, height); 
    spawning_pool.set(player, components::Physics{coord: rooms[0].center()});
    add_down_stairs(rooms[1].center(), spawning_pool);
    for room in rooms.iter().skip(2) {
        let entity = if rand::random::<bool>() {
            let c = thread_rng().gen_range(0, creatures.len());
            create_creature(&creatures[c as usize], room.center(), spawning_pool)
        } else {
            add_item(room.center(), spawning_pool)
        };
        scheduler.add_entity(entity, spawning_pool);
    }

    let map = Map::new(&cells);

    for x in 1..(map.dimensions.x - 1) {
        for y in 1..(map.dimensions.y - 1) {
            if map.get_cell(x as i32, y as i32).tile_type != TileType::Floor {
                continue;
            }
            let neighbours = map.get_neigbours(x as i32, y as i32);
            let floors: Vec<&(Point, &Cell)> = neighbours.iter().filter(|v| v.1.tile_type == TileType::Floor).collect();
            if floors.len() == 4 && rand::random::<f32>() < 0.2 {
                add_door((x, y).into(), spawning_pool);
            }
        }
    }

    map
}

fn add_door(pos: Point, spawning_pool: &mut components::SpawningPool) -> EntityId {
    let door = spawning_pool.spawn_entity();
    spawning_pool.set(door, components::Visual{glyph: '+', color: colors::WHITE});
    spawning_pool.set(door, components::Physics{coord: (pos.x, pos.y).into()});
    spawning_pool.set(door, components::Information{name: "closed door".to_string()});
    spawning_pool.set(door, components::Flags{block_sight: true, solid: true});
    spawning_pool.set(door, components::Door{opened: false});
    door
}

fn add_down_stairs(pos: Point, spawning_pool: &mut components::SpawningPool) -> EntityId {
    let stairs = spawning_pool.spawn_entity();
    spawning_pool.set(stairs, components::Visual{glyph: '<', color: colors::WHITE});
    spawning_pool.set(stairs, components::Physics{coord: (pos.x, pos.y).into()});
    spawning_pool.set(stairs, components::Information{name: "down stairs".to_string()});
    spawning_pool.set(stairs, components::Flags{block_sight: false, solid: false});
    stairs
}


fn add_item(pos: Point, spawning_pool: &mut components::SpawningPool) -> EntityId {
    let chances = &mut [
        Weighted {
            weight: 30,
            item: "healing"
        },
        Weighted {
            weight: 30,
            item: "scroll"
        },
        Weighted {
            weight: 50,
            item: "confuse"
        },
        Weighted {
            weight: 10,
            item: "sword"
        },
        Weighted {
            weight: 10,
            item: "shield"
        },
    ];

    let choice = WeightedChoice::new(chances);

    match choice.ind_sample(&mut thread_rng()) {
        "healing" => add_healing_potion(pos, spawning_pool),
        "scroll" => add_lightning_scroll(pos, spawning_pool),
        "confuse" => add_confusion_scroll(pos, spawning_pool),
        "sword" => add_sword(pos, spawning_pool),
        "shield" => add_shield(pos, spawning_pool),
        _ => panic!()
    }
}

fn add_sword(pos: Point, spawning_pool: &mut components::SpawningPool) -> EntityId {
    let item = spawning_pool.spawn_entity();
    spawning_pool.set(item, components::Visual{glyph: '/', color: colors::LIGHT_CYAN});
    spawning_pool.set(item, components::Physics{coord: pos});
    spawning_pool.set(item, components::Flags{block_sight: false, solid: false});
    spawning_pool.set(item, components::Information{name: "sword".to_string()});
    spawning_pool.set(item, components::Item{
        on_use: None,
        equip: Some(components::EquipmentSlot::RightHand),
        statistics_bonus: Some(components::StatisticsBonus{
            strength: 5,
            defense: 0
        })
    });
    item
}

fn add_confusion_scroll(pos: Point, spawning_pool: &mut components::SpawningPool) -> EntityId {
    let item = spawning_pool.spawn_entity();
    spawning_pool.set(item, components::Visual{glyph: '?', color: tcod::colors::Color{r: 130, g: 50, b: 130}});
    spawning_pool.set(item, components::Physics{coord: pos});
    spawning_pool.set(item, components::Flags{block_sight: false, solid: false});
    spawning_pool.set(item, components::Information{name: "scroll of confusion".to_string()});
    spawning_pool.set(item, components::Item{
        on_use: Some(components::OnUseCallback::Spell(spells::Spells::Confusion)),
        equip: None,
        statistics_bonus: None
    });
    item
}

fn add_lightning_scroll(pos: Point, spawning_pool: &mut components::SpawningPool) -> EntityId {
    let item = spawning_pool.spawn_entity();
    spawning_pool.set(item, components::Visual{glyph: '?', color: tcod::colors::Color{r: 0, g: 150, b: 180}});
    spawning_pool.set(item, components::Physics{coord: pos});
    spawning_pool.set(item, components::Flags{block_sight: false, solid: false});
    spawning_pool.set(item, components::Information{name: "scroll of lightning".to_string()});
    spawning_pool.set(item, components::Item{
        on_use: Some(components::OnUseCallback::Spell(spells::Spells::LightningStrike)),
        equip: None,
        statistics_bonus: None
    });
    item
}

fn add_shield(pos: Point, spawning_pool: &mut components::SpawningPool) -> EntityId {
    let item = spawning_pool.spawn_entity();
    spawning_pool.set(item, components::Visual{glyph: ')', color: colors::LIGHTEST_CYAN});
    spawning_pool.set(item, components::Physics{coord: pos});
    spawning_pool.set(item, components::Flags{block_sight: false, solid: false});
    spawning_pool.set(item, components::Information{name: "buckler".to_string()});
    spawning_pool.set(item, components::Item{
        on_use: None,
        equip: Some(components::EquipmentSlot::LeftHand),
        statistics_bonus: Some(components::StatisticsBonus{
            strength: 0,
            defense: 3
        })
    });
    item
}

fn add_healing_potion(pos: Point, spawning_pool: &mut components::SpawningPool) -> EntityId {
    let item = spawning_pool.spawn_entity();
    spawning_pool.set(item, components::Visual{glyph: '!', color: colors::PINK});
    spawning_pool.set(item, components::Physics{coord: pos});
    spawning_pool.set(item, components::Flags{block_sight: false, solid: false});
    spawning_pool.set(item, components::Information{name: "potion of healing".to_string()});
    spawning_pool.set(item, components::Item{
        on_use: Some(components::OnUseCallback::SelfHeal),
        equip: None,
        statistics_bonus: None
    });
    item
}

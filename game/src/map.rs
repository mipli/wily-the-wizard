use tcod;
use tcod::colors;

use rand;
use rand::*;
use rand::distributions::{IndependentSample, Weighted, WeightedChoice};

use spells;
use spatial::*;
use map_generator::{Map as GeneratedMap, bsp};
use components;
use geo::*;
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
    pub fn get_render_info(&self) -> (char, tcod::colors::Color, tcod::colors::Color) {
        match self.tile_type {
            TileType::Wall => ('#', colors::DARK_GREY, tcod::colors::Color{r: 0, g: 20, b: 35}),
            TileType::Floor => ('.', colors::DARK_GREY, tcod::colors::Color{r: 0, g: 10, b: 20})
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Map {
    pub dimensions: Point,
    pub cells: Vec<Cell>,
}

impl Map {
    pub fn new(m: &GeneratedMap) -> Map {
        let mut cells = vec![];

        for v in &m.data {
            let tile_type = match v {
                0 => TileType::Wall,
                _ => TileType::Floor
            };
            cells.push(Cell{
                tile_type,
                blocks_movement: tile_type == TileType::Wall,
                blocks_sight: tile_type == TileType::Wall
            });
        }

        Map {
            dimensions: (m.width, m.height).into(),
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

    pub fn is_floor(&self, position: Point) -> bool {
        self.get_cell(position.x, position.y).tile_type == TileType::Floor
    }
}

pub fn can_walk(position: Point, grid: &SpatialTable, map: &Map) -> bool {
    match grid.get(position) {
        Some(cell) => {
            !cell.solid && map.is_floor(position)
        },
        None => {
            false
        }
    }
}

pub fn create_map(player: EntityId, width: i32, height: i32, spawning_pool: &mut components::SpawningPool, scheduler: &mut Scheduler, seed: Option<[u32; 4]>) -> Map{
    let creatures = load_creatures();
    let mut rng: XorShiftRng = if let Some(seed) = seed {
        SeedableRng::from_seed(seed)
    } else {
        rand::weak_rng()
    };

    let generated = bsp::generate(width, height, 5, &mut rng);
    let map = Map::new(&generated);

    spawning_pool.set(player, components::Physics{coord: generated.rooms[0].center()});
    add_down_stairs(generated.stairs.unwrap(), spawning_pool);
    for room in generated.rooms.iter().skip(1) {
        let p = rng.gen::<f32>();
        if p < 0.6 {
            add_monsters(room, &creatures, scheduler, width, height, spawning_pool, &mut rng);
        } else if p < 0.8 {
            let entity = add_item(room.center(), spawning_pool, &mut rng);
            scheduler.schedule_entity(entity, 0, spawning_pool);
        }
    }

    for x in 0..generated.width {
        for y in 0..generated.height {
            if generated.get(x, y) == 2 {
                add_door((x, y).into(), spawning_pool);
            }
        }
    }

    map
}

#[derive(Copy, Clone, PartialEq, Debug, Serialize, Deserialize)]
enum RoomDifficulty {
    Easy,
    Normal,
    Difficult
}

fn add_monsters<T: Rng>(room: &Rect, creatures: &Vec<CreatureData>, scheduler: &mut Scheduler, width: i32, height: i32, spawning_pool: &mut components::SpawningPool, rng: &mut T) {
    let chances = &mut [
        Weighted {
            weight: 5,
            item: RoomDifficulty::Easy
        },
        Weighted {
            weight: 3,
            item: RoomDifficulty::Normal
        },
        Weighted {
            weight: 1,
            item: RoomDifficulty::Difficult
        }
    ];

    let choice = WeightedChoice::new(chances);

    match choice.ind_sample(rng) {
        RoomDifficulty::Easy => {
            add_creature(&creatures[0], room, width, height, scheduler, spawning_pool, rng);
            add_creature(&creatures[0], room, width, height, scheduler, spawning_pool, rng);
        },
        RoomDifficulty::Normal => {
            add_creature(&creatures[1], room, width, height, scheduler, spawning_pool, rng);
            add_creature(&creatures[0], room, width, height, scheduler, spawning_pool, rng);
            add_creature(&creatures[0], room, width, height, scheduler, spawning_pool, rng);
        },
        RoomDifficulty::Difficult => {
            add_creature(&creatures[0], room, width, height, scheduler, spawning_pool, rng);
            add_creature(&creatures[1], room, width, height, scheduler, spawning_pool, rng);
            add_creature(&creatures[1], room, width, height, scheduler, spawning_pool, rng);
            add_creature(&creatures[2], room, width, height, scheduler, spawning_pool, rng);
        }
    }
}

fn add_creature<T: Rng>(creature: &CreatureData, room: &Rect, width: i32, height: i32, scheduler: &mut Scheduler, spawning_pool: &mut components::SpawningPool, rng: &mut T) {
    if let Some(point) = get_empty_spot(room, spawning_pool, rng) {
        let creature = create_creature(creature, point, width, height, spawning_pool);
        scheduler.schedule_entity(creature, 0, spawning_pool);
    }
}

fn get_empty_spot<T: Rng>(room: &Rect, spawning_pool: &mut components::SpawningPool, rng: &mut T) -> Option<Point> {
    let mut iter = 0;

    while iter < 10 {
        iter += 1;
        let x = room.x1 + rng.gen_range(0, room.width);
        let y = room.y1 + rng.gen_range(0, room.height);
        let point = Point::new(x, y);
        if !spawning_pool.get_all::<components::Physics>().iter().any(|(_, phys)| phys.coord == point) {
            return Some(point);
        }
    }

    None
}


fn add_door(pos: Point, spawning_pool: &mut components::SpawningPool) -> EntityId {
    let door = spawning_pool.spawn_entity();
    spawning_pool.set(door, components::Visual{always_display: true, glyph: '+', color: colors::WHITE});
    spawning_pool.set(door, components::Physics{coord: (pos.x, pos.y).into()});
    spawning_pool.set(door, components::Information{name: "closed door".to_string()});
    spawning_pool.set(door, components::Flags{block_sight: true, solid: true});
    spawning_pool.set(door, components::Door{opened: false});
    door
}

fn add_down_stairs(pos: Point, spawning_pool: &mut components::SpawningPool) -> EntityId {
    let stairs = spawning_pool.spawn_entity();
    spawning_pool.set(stairs, components::Visual{always_display: true, glyph: '<', color: colors::WHITE});
    spawning_pool.set(stairs, components::Physics{coord: (pos.x, pos.y).into()});
    spawning_pool.set(stairs, components::Information{name: "down stairs".to_string()});
    spawning_pool.set(stairs, components::Flags{block_sight: false, solid: false});
    stairs
}

fn add_item<T: Rng>(pos: Point, spawning_pool: &mut components::SpawningPool, rng: &mut T) -> EntityId {
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

    match choice.ind_sample(rng) {
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
    spawning_pool.set(item, components::Visual{always_display: false, glyph: '/', color: colors::LIGHT_CYAN});
    spawning_pool.set(item, components::Physics{coord: pos});
    spawning_pool.set(item, components::Flags{block_sight: false, solid: false});
    spawning_pool.set(item, components::Information{name: "sword".to_string()});
    spawning_pool.set(item, components::Item{
        on_use: None,
        equip: Some(components::EquipmentSlot::RightHand),
        kind: components::ItemKind::Equipment,
        statistics_bonus: Some(components::StatisticsBonus{
            strength: 5,
            defense: 0
        })
    });
    item
}

fn add_confusion_scroll(pos: Point, spawning_pool: &mut components::SpawningPool) -> EntityId {
    let item = spawning_pool.spawn_entity();
    spawning_pool.set(item, components::Visual{always_display: false, glyph: '?', color: tcod::colors::Color{r: 130, g: 50, b: 130}});
    spawning_pool.set(item, components::Physics{coord: pos});
    spawning_pool.set(item, components::Flags{block_sight: false, solid: false});
    spawning_pool.set(item, components::Information{name: "scroll of confusion".to_string()});
    spawning_pool.set(item, components::Item{
        on_use: Some(components::OnUseCallback::Spell(spells::Spells::Confusion)),
        equip: None,
        kind: components::ItemKind::Scroll,
        statistics_bonus: None
    });
    item
}

fn add_lightning_scroll(pos: Point, spawning_pool: &mut components::SpawningPool) -> EntityId {
    let item = spawning_pool.spawn_entity();
    spawning_pool.set(item, components::Visual{always_display: false, glyph: '?', color: tcod::colors::Color{r: 0, g: 150, b: 180}});
    spawning_pool.set(item, components::Physics{coord: pos});
    spawning_pool.set(item, components::Flags{block_sight: false, solid: false});
    spawning_pool.set(item, components::Information{name: "scroll of lightning".to_string()});
    spawning_pool.set(item, components::Item{
        on_use: Some(components::OnUseCallback::Spell(spells::Spells::LightningStrike)),
        equip: None,
        kind: components::ItemKind::Scroll,
        statistics_bonus: None
    });
    item
}

fn add_shield(pos: Point, spawning_pool: &mut components::SpawningPool) -> EntityId {
    let item = spawning_pool.spawn_entity();
    spawning_pool.set(item, components::Visual{always_display: false, glyph: ')', color: colors::LIGHTEST_CYAN});
    spawning_pool.set(item, components::Physics{coord: pos});
    spawning_pool.set(item, components::Flags{block_sight: false, solid: false});
    spawning_pool.set(item, components::Information{name: "buckler".to_string()});
    spawning_pool.set(item, components::Item{
        on_use: None,
        equip: Some(components::EquipmentSlot::LeftHand),
        kind: components::ItemKind::Equipment,
        statistics_bonus: Some(components::StatisticsBonus{
            strength: 0,
            defense: 3
        })
    });
    item
}

fn add_healing_potion(pos: Point, spawning_pool: &mut components::SpawningPool) -> EntityId {
    let item = spawning_pool.spawn_entity();
    spawning_pool.set(item, components::Visual{always_display: false, glyph: '!', color: colors::PINK});
    spawning_pool.set(item, components::Physics{coord: pos});
    spawning_pool.set(item, components::Flags{block_sight: false, solid: false});
    spawning_pool.set(item, components::Information{name: "potion of healing".to_string()});
    spawning_pool.set(item, components::Item{
        on_use: Some(components::OnUseCallback::Spell(spells::Spells::Heal)),
        equip: None,
        kind: components::ItemKind::Potion,
        statistics_bonus: None
    });
    item
}

use tcod::colors;
use tcod;

use spawning_pool::EntityId;

use render;
use components;
use systems;

use map::*;
use geo::*;
use spatial::*;
use ai;
use actions::*;
use rules::*;
use utils;
use consts::*;
use messages::*;
use spells;
use scheduler::{Scheduler};

#[derive(Serialize, Deserialize)]
pub struct GameState {
    pub spawning_pool: components::SpawningPool,
    pub player: EntityId,
    pub map: Map,
    pub scheduler: Scheduler,
    pub spatial_table: SpatialTable,
    pub messages: Messages,
}

impl GameState {
    pub fn new() -> GameState {
        let mut spawning_pool = components::SpawningPool::new();
        let player = create_player(&mut spawning_pool, MAP_WIDTH, MAP_HEIGHT);
        let mut scheduler = Scheduler::new();
        scheduler.schedule_entity(player, 0, &spawning_pool);
        let map = create_map(player, MAP_WIDTH, MAP_HEIGHT, &mut spawning_pool, &mut scheduler, None);

        let mut spatial_table = SpatialTable::new(map.dimensions.x, map.dimensions.y);
        spatial_table.reset(&spawning_pool);
        spatial_table.dirty = true;

        GameState {
            spawning_pool,
            spatial_table,
            player,
            map,
            scheduler,
            messages: vec![]
        }
    }

    pub fn new_level(&mut self) {
        for cell in &self.spatial_table.cells {
            for entity in &cell.entities {
                if entity != &self.player {
                    self.spawning_pool.remove_entity(*entity);
                }
            }
        }

        let map = create_map(self.player, MAP_WIDTH, MAP_HEIGHT, &mut self.spawning_pool, &mut self.scheduler, None);
        self.spatial_table.reset(&self.spawning_pool);
        self.map = map;

        if let Some(memory) = self.spawning_pool.get_mut::<components::MapMemory>(self.player) {
            memory.reset();
        }
    }
}

pub struct Game {
    pub state: GameState,
    pub current_action: Option<Action>,
    pub action_queue: Vec<Action>,
    pub reaction_queue: Vec<Action>,
    pub rejection_queue: Vec<Action>,
    pub fov: tcod::map::Map,
    pub tick_time: i32,
    pub systems: systems::DurationSystem
}

pub enum WaitResult {
    Wait,
    RequireTarget{action: Action}
}

pub enum TickResult {
    Passed,
    Wait(WaitResult)
}

#[derive(Debug, Eq, PartialEq)]
enum ActionTickResult {
    RequireInformation,
    Performed{time: i32},
    Pass
}

impl Game {
    pub fn game_tick(&mut self, actions: Vec<Action>, animations: &mut Vec<render::Animation>) -> TickResult {
        self.state.scheduler.tick(&self.state.spawning_pool);
        self.systems.run(&mut self.state);
        if self.state.spawning_pool.get::<components::MapMemory>(self.state.scheduler.get_current()).is_some() {
            self.update_fov();
        }
        let actions = self.get_actions(actions);
        if let Some(mut actions) = actions {
            for action in actions.drain(..) {
                self.action_queue.insert(0, action);
            }
        }
        self.current_action = None;
        if !self.action_queue.is_empty() && self.action_queue[0].command != Command::Wait {
            println!("Actions: {:?}", self.action_queue);
        }

        let mut performed_action = false;
        let mut require_information = false;
        while !self.action_queue.is_empty() && !require_information {
            if self.current_action.is_none() {
                self.current_action = Some(self.action_queue.remove(0));
            }
            let res = self.action_tick(animations);
            require_information = res == ActionTickResult::RequireInformation;
            if let ActionTickResult::Performed{time} = res {
                performed_action = true;
                self.tick_time += time;
            }
            if !require_information {
                self.current_action = None;
            }
        }
        if require_information {
            if let Some(ref action) = self.current_action {
                TickResult::Wait(WaitResult::RequireTarget{action: action.clone()})
            } else {
                panic!("TickResult waiting without game.current_action");
            }
        } else if performed_action {
            let entity = self.state.scheduler.get_current();
            self.state.scheduler.schedule_entity(entity, self.tick_time, &self.state.spawning_pool);
            self.tick_time = 0;
            self.current_action = None;
            self.rejection_queue = vec![];
            self.reaction_queue = vec![];
            self.state.spatial_table.reset(&self.state.spawning_pool);
            TickResult::Passed
        } else {
            TickResult::Wait(WaitResult::Wait)
        }
    }

    fn get_actions(&mut self, actions: Vec<Action>) -> Option<Vec<Action>>{
        let pre_action = systems::confusion(self.state.scheduler.get_current(), &mut self.state);
        match pre_action {
            Some(action) => {
                Some(vec![action])
            },
            None => self.get_entity_actions(actions)
        }
    }

    fn action_tick(&mut self, animations: &mut Vec<render::Animation>) -> ActionTickResult {
        let mut require_information = false;
        let mut performed_action = false;
        let mut used_time = 0;
        if let Some(ref mut action) = self.current_action {
            require_information = check_require_information(action);
            if !require_information {
                let action_status = apply_rules(action, &self.state, &mut self.rejection_queue, &mut self.reaction_queue);
                match action_status {
                    ActionStatus::Accept => {
                        let action_result = perform_action(action, &mut self.state);
                        if action_result == ActionResult::Failed {
                            self.reaction_queue = vec![];
                            performed_action = false;
                        } else {
                            performed_action = performed_action || match action_result {
                                ActionResult::Performed{time} => {
                                    used_time += time;
                                    true
                                },
                                _ => false
                            };
                        }
                        if performed_action {
                            animate_action(action, animations, &self.state.spawning_pool);
                            if let Some(reaction) = self.reaction_queue.pop() {
                                self.action_queue.insert(0, reaction);
                            }
                        } else {
                            self.reaction_queue = vec![];
                        }
                    }
                    ActionStatus::Reject => {
                        self.reaction_queue = vec![];
                        self.action_queue = self.rejection_queue.drain(..).collect();
                        self.action_queue.reverse();
                    }
                };
            }
        }
        if require_information {
            ActionTickResult::RequireInformation
        } else if performed_action {
            ActionTickResult::Performed{time: used_time}
        } else {
            ActionTickResult::Pass
        }
    }

    fn get_entity_actions(&mut self, actions: Vec<Action>) -> Option<Vec<Action>> {
        use components::*;
        let ai = match self.state.spawning_pool.get::<Controller>(self.state.scheduler.get_current()) {
            Some(controller) => Some(controller.ai),
            None => None
        };
        if let Some(ai) = ai {
            return match ai {
                AI::Player => {
                    if !actions.is_empty() {
                        Some(actions)
                    } else {
                        None
                    }
                },
               AI::Basic => ai::perform_basic_ai(self.state.scheduler.get_current(), &mut self.state),
               AI::SpellCaster => ai::perform_spell_ai(self.state.scheduler.get_current(), &mut self.state),
            }
        } else {
            panic!("Trying to get entity actions on entity without controller");
        }
    }

    pub fn update_fov(&mut self) {
        let entity = self.state.scheduler.get_current();
        let coord = match self.state.spawning_pool.get::<components::Physics>(entity) {
            Some(physics) => physics.coord,
            None => return
        };
        if self.state.spawning_pool.get::<components::MapMemory>(entity).is_some() {
            self.calculate_fov(coord.x, coord.y, 10);
        }
        if let Some(map_memory) = self.state.spawning_pool.get_mut::<components::MapMemory>(entity) {
            map_memory.clear_visible();
            for x in 0..map_memory.dimensions.x {
                for y in 0..map_memory.dimensions.y {
                    if self.fov.is_in_fov(x, y) {
                        map_memory.explore(x, y);
                        map_memory.set_visible(x, y, true);
                    }
                }
            }
        }
    }

    fn calculate_fov(&mut self, x: i32, y: i32, sight_radius: i32) {
        self.fov.compute_fov(x, y, sight_radius, true, tcod::map::FovAlgorithm::Basic);
    }
}

fn check_require_information(action: &Action) -> bool {
    match action.command {
        Command::CastSpell{ref spell} => {
            spell.target == spells::SpellTargetType::Entity && action.target.is_none()
        },
        _ => {
            false
        }
    }
}

fn animate_action(action: &Action, animations: &mut Vec<render::Animation>, spawning_pool: &components::SpawningPool) {
    match action.command {
        Command::LightningStrike{..} => {
            if let Some(target) = action.target {
                if let Some(pos) = utils::get_position(target, spawning_pool) {
                    if let Some(glyph) = utils::get_glyph(target, spawning_pool) {
                        animations.push(render::Animation::new(
                            render::AnimationAnchor::Position{point: pos},
                            200.0, // time
                            Some(2000.0), // duration
                            vec![None, Some((glyph, tcod::colors::LIGHT_SKY))]
                        ));
                    }
                }
            }
        },
        Command::TakeDamage{..} => {
            if let Some(target) = action.target {
                if let Some(glyph) = utils::get_glyph(target, spawning_pool) {
                    animations.push(render::Animation::new(
                        render::AnimationAnchor::Entity{entity: target},
                        0.0, // time
                        Some(300.0), // duration
                        vec![Some((glyph, tcod::colors::RED))]
                    ));
                }
            }
        },
        _ => {}
    }
}

pub fn get_item_at(position: Point, game_state: &GameState) -> Option<EntityId> {
    let cell = game_state.spatial_table.get(position).unwrap();
    for id in &cell.entities {
        if game_state.spawning_pool.get::<components::Item>(*id).is_some() {
            return Some(*id);
        }
    }
    None
}

pub fn get_entity_position(entity: EntityId, game_state: &GameState) -> Option<Point> {
    let pos_result = game_state.spawning_pool.get::<components::Physics>(entity)?;
    Some(pos_result.coord)
}

fn create_player(spawning_pool: &mut components::SpawningPool, width: i32, height: i32) -> EntityId {
    let player = spawning_pool.spawn_entity();
    spawning_pool.set(player, components::Visual{always_display: false, glyph: '@', color: colors::WHITE});
    spawning_pool.set(player, components::Physics{coord: (0,0).into()});
    spawning_pool.set(player, components::Controller{ai: components::AI::Player});
    spawning_pool.set(player, components::Information{name: "player".to_string()});
    spawning_pool.set(player, components::Flags{solid: true, block_sight: false});
    spawning_pool.set(player, components::Inventory{items: vec![]});
    spawning_pool.set(player, components::MapMemory::new(width, height));
    spawning_pool.set(player, components::Equipment{items: Default::default()});
    spawning_pool.set(player, components::Stats{
        faction: components::Faction::Player,
        max_health: 10,
        health: 10,
        strength: 5,
        defense: 3
    });
    player
}

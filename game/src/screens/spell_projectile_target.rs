use tcod::line::{Line};
use spawning_pool::{EntityId};
use crate::screens::*;
use crate::components;
use tcod;
use tcod::input::{KeyCode};
use crate::render;
use geo::{Point};
use crate::map::Map;

pub struct SpellProjectileTargetScreen {
    exit: bool,
    selected: Option<EntityId>,
    entities: Vec<EntityId>,
    target: Option<Point>,
    target_index: usize,
    target_id: Option<EntityId>,
    origin: Point,
    range: i32,
    ray: Vec<(Point, bool)>,
    callback: Box<Fn(EntityId, &mut GameState, &mut Vec<Action>)>
}

impl SpellProjectileTargetScreen {
    pub fn new(origin: Point, range: i32, state: &GameState, callback: Box<Fn(EntityId, &mut GameState, &mut Vec<Action>)>) -> Self {
        let mut me = SpellProjectileTargetScreen {
            exit: false,
            entities: vec![],
            selected: None,
            target_index: 0,
            target_id: None,
            target: None,
            ray: vec![],
            range,
            origin,
            callback
        };
        me.init(state);
        me
    }

    fn init(&mut self, state: &GameState) {
        let ents = state.spatial_table.get_by_proximity(self.origin, self.range);
        if let Some(map_memory) = state.spawning_pool.get::<components::MapMemory>(state.player) {
            self.entities = ents.iter()
                .filter(|(pos, _)| map_memory.is_visible(pos.x, pos.y))
                .filter(|(_, id)| state.spawning_pool.get::<components::Stats>(*id).is_some())
                .map(|(_, id)| *id)
                .collect();
            if !self.entities.is_empty() {
                self.entities = self.entities[1..].to_vec();
            }
        }
        if !self.entities.is_empty() {
            if let Some(physics) = state.spawning_pool.get::<components::Physics>(self.entities[self.target_index]) {
                self.target = Some(physics.coord);
                self.target_id = Some(self.entities[self.target_index]);
            }
        } else {
            self.exit = true;
        }
    }

    fn position_is_valid(&self, pos: Point, memory: &components::MapMemory, map: &Map) -> bool {
        self.origin.distance(pos) <= self.range as f32 && map.is_floor(pos) && memory.is_visible(pos.x, pos.y)
    }

    fn update_ray(&mut self, state: &mut GameState) {
        if let Some(memory) = state.spawning_pool.get::<components::MapMemory>(state.player) {
            if let Some(target) = self.target {
                let line = Line::new((self.origin.x, self.origin.y), (target.x, target.y));
                let mut valid = true;
                for (x, y) in line {
                    valid = valid && self.position_is_valid((x, y).into(), memory, &state.map);
                    self.ray.push(((x, y).into(), valid));
                }
            }
        }
    }
}

impl Screen for SpellProjectileTargetScreen {
    fn should_discard(&self, _state: &mut GameState) -> bool {
        self.exit
    }

    fn new_screens(&mut self, _state: &mut GameState) -> Vec<ScreenPointer> {
        vec![]
    }

    fn render(&mut self, _delta: f64, _state: &mut GameState, _fov: &tcod::map::Map, tcod: &mut render::Tcod) -> (ScreenResult, Option<ModularWindow>) {
        for (pos, valid) in &self.ray {
            let color = if *valid {
                tcod::colors::LIGHT_CYAN
            } else {
                tcod::colors::LIGHT_RED
            };
            tcod.add_animation(render::Animation::new(
                render::AnimationAnchor::Position{point: *pos},
                5.0, // time
                Some(10.0),
                vec![Some(('+', color))]
            ));
        }
        (ScreenResult::PassThrough, None)
    }

    fn tick(&mut self, state: &mut GameState, _tcod: &mut render::Tcod, actions: &mut Vec<Action>) -> ScreenResult {
        if self.ray.is_empty() {
            self.update_ray(state);
        }
        if let Some(selected) = self.selected {
            (self.callback)(selected, state, actions);
        }
        if self.exit && self.selected.is_none() {
            actions.push(Action {
                actor: Some(state.player),
                target: None,
                command: Command::Abort
            });
        }
        ScreenResult::Stop
    }

    fn handle_input(&mut self, input: &Input, state: &mut GameState) -> ScreenResult {
        match input.key {
            Key { code: KeyCode::Escape, .. } | Key { code: KeyCode::Text, printable: 'q', .. } => {
                self.exit = true;
            },
            Key { code: KeyCode::Enter, .. } => {
                self.selected = self.target_id;
                self.exit = true;
            },
            Key { code: KeyCode::Tab, .. } => {
                self.ray.clear();
                self.target_index = (self.target_index + 1) % self.entities.len();
                if !self.entities.is_empty() {
                    if let Some(physics) = state.spawning_pool.get::<components::Physics>(self.entities[self.target_index]) {
                        self.target = Some(physics.coord);
                        self.target_id = Some(self.entities[self.target_index]);
                    }
                }
            }
            _ => {}
        };
        ScreenResult::Stop
    }
}

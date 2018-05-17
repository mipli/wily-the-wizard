use spawning_pool::{EntityId};
use screens::*;
use components;
use tcod;
use utils;
use tcod::input::{KeyCode};
use render;

pub struct TargetScreen {
    exit: bool,
    entities: Vec<EntityId>,
    target: Option<Point>,
    target_index: usize,
    target_id: Option<EntityId>,
    origin: Point
}

impl TargetScreen {
    pub fn new(origin: Point, state: &GameState) -> Self {
        let mut me = TargetScreen {
            exit: false,
            entities: vec![],
            target_index: 0,
            target_id: None,
            target: None,
            origin
        };
        me.init(state);
        me
    }

    fn init(&mut self, state: &GameState) {
        let ents = state.spatial_table.get_by_proximity(self.origin, 10);
        if let Some(map_memory) = state.spawning_pool.get::<components::MapMemory>(state.player) {
            self.entities = ents.iter()
                .filter(|(pos, _)| map_memory.is_visible(pos.x, pos.y))
                .map(|(_, id)| *id)
                .collect();
            self.entities = self.entities[1..].to_vec();
        }
        if !self.entities.is_empty() {
            if let Some(physics) = state.spawning_pool.get::<components::Physics>(self.entities[self.target_index]) {
                self.target = Some(physics.coord);
                self.target_id = Some(self.entities[self.target_index]);
            }
        }
    }

}

impl Screen for TargetScreen {
    fn should_discard(&self, _state: &mut GameState) -> bool {
        self.exit
    }

    fn new_screens(&mut self, _state: &mut GameState) -> Vec<ScreenPointer> {
        vec![]
    }

    fn render(&mut self, _delta: f64, state: &mut GameState, _fov: &tcod::map::Map, tcod: &mut render::Tcod) -> (ScreenResult, Option<ModularWindow>) {
        if let Some(target_id) = self.target_id {
            let description = utils::describe_entity(target_id, &state.spawning_pool);
            let mut root = Offscreen::new(description.len() as i32 + 2, 3);
            root.print_rect_ex(
                1,
                1, 
                description.len() as i32,
                1,
                BackgroundFlag::None,
                TextAlignment::Left,
                description
            );
            if let Some(target) = self.target {
                tcod.add_animation(render::Animation::new(
                    render::AnimationAnchor::Position{point: target},
                    5.0, // time
                    Some(10.0),
                    vec![Some(('X', tcod::colors::LIGHT_CYAN))]
                ));
            }
            (ScreenResult::PassThrough, Some(ModularWindow{screen: root, alpha: 0.7, pos: ModularWindowPosition::Position{point: (2, 2).into()}}))
        } else {
            (ScreenResult::PassThrough, None)
        }
    }

    fn tick(&mut self, _state: &mut GameState, _tcod: &mut render::Tcod, _actions: &mut Vec<Action>) -> ScreenResult {
        ScreenResult::Stop
    }

    fn handle_input(&mut self, input: &Input, state: &mut GameState) -> ScreenResult {
        match input.key {
            Key { code: KeyCode::Escape, .. } | Key { printable: 'q', .. } => {
                self.exit = true;
            },
            Key { code: KeyCode::Tab, .. } => {
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


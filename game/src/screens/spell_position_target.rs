use crate::screens::*;
use crate::components;
use tcod;
use tcod::input::{KeyCode};
use crate::render;
use geo::{Point};
use crate::map::Map;

pub struct SpellPositionTargetScreen {
    exit: bool,
    valid: bool,
    position: Point,
    selected: bool,
    origin: Point,
    range: f32,
    callback: Box<Fn(Point, &mut GameState, &mut Vec<Action>)>
}

impl SpellPositionTargetScreen {
    pub fn new(origin: Point, range: i32, _state: &GameState, callback: Box<Fn(Point, &mut GameState, &mut Vec<Action>)>) -> Self {
        SpellPositionTargetScreen {
            exit: false,
            valid: false,
            selected: false,
            position: origin,
            range: range as f32,
            origin,
            callback
        }
    }

    fn position_is_valid(&self, memory: &components::MapMemory, map: &Map) -> bool {
        self.origin.distance(self.position) <= self.range && map.is_floor(self.position) && memory.is_visible(self.position.x, self.position.y)
    }
}

impl Screen for SpellPositionTargetScreen {
    fn should_discard(&self, _state: &mut GameState) -> bool {
        self.exit
    }

    fn new_screens(&mut self, _state: &mut GameState) -> Vec<ScreenPointer> {
        vec![]
    }

    fn render(&mut self, _delta: f64, _state: &mut GameState, _fov: &tcod::map::Map, tcod: &mut render::Tcod) -> (ScreenResult, Option<ModularWindow>) {
        let color = if self.valid {
            tcod::colors::LIGHT_CYAN
        } else {
            tcod::colors::LIGHT_RED
        };
        tcod.add_animation(render::Animation::new(
            render::AnimationAnchor::Position{point: self.position},
            5.0, // time
            Some(10.0),
            vec![Some(('X', color))]
        ));
        (ScreenResult::PassThrough, None)
    }

    fn tick(&mut self, state: &mut GameState, _tcod: &mut render::Tcod, actions: &mut Vec<Action>) -> ScreenResult {
        if let Some(memory) = state.spawning_pool.get::<components::MapMemory>(state.player) {
            self.valid = self.position_is_valid(memory, &state.map);
        } else {
            self.valid = false;
        }
        if self.selected && self.valid {
            (self.callback)(self.position, state, actions);
            self.exit = true;
        }
        if self.exit && !self.selected {
            actions.push(Action::new(
                Some(state.player),
                None,
                Command::Abort
            ));
        }
        self.selected = false;
        ScreenResult::Stop
    }

    fn handle_input(&mut self, input: &Input, _state: &mut GameState) -> ScreenResult {
        match input.key {
            Key { code: KeyCode::Escape, .. } | Key { code: KeyCode::Text, printable: 'q', .. } => {
                self.exit = true;
            },
            Key { code: KeyCode::Enter, .. } => {
                self.selected = true;
            },
            Key { code: KeyCode::Up, .. } | Key { code: KeyCode::Text, printable: 'k', .. } => {
                self.position += (0, -1);
            },
            Key { code: KeyCode::Text, printable: 'u', .. } => {
                self.position += (1, -1);
            },
            Key { code: KeyCode::Right, .. } | Key { code: KeyCode::Text, printable: 'l', .. } => {
                self.position += (1, 0);
            },
            Key { code: KeyCode::Text, printable: 'n', .. } => {
                self.position += (1, 1);
            },
            Key { code: KeyCode::Down, .. } | Key { code: KeyCode::Text, printable: 'j', .. } => {
                self.position += (0, 1);
            },
            Key { code: KeyCode::Text, printable: 'b', .. } => {
                self.position += (-1, 1);
            },
            Key { code: KeyCode::Left, .. } | Key { code: KeyCode::Text, printable: 'h', .. } => {
                self.position += (-1, 0);
            },
            Key { code: KeyCode::Text, printable: 'y', .. } => {
                self.position += (-1, -1);
            },
            _ => {}
        };
        ScreenResult::Stop
    }
}

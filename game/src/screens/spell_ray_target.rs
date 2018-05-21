use tcod::line::{Line};
use screens::*;
use components;
use tcod;
use tcod::input::{KeyCode};
use render;
use geo::{Point};
use map::Map;

pub struct SpellRayTargetScreen {
    exit: bool,
    valid: bool,
    position: Point,
    selected: bool,
    origin: Point,
    range: f32,
    callback: Box<Fn(Point, &mut GameState, &mut Vec<Action>)>
}

impl SpellRayTargetScreen {
    pub fn new(origin: Point, range: i32, state: &GameState, callback: Box<Fn(Point, &mut GameState, &mut Vec<Action>)>) -> Self {
        let position = get_closest_entity_position(origin, range, state).unwrap_or(origin);
        SpellRayTargetScreen {
            exit: false,
            valid: false,
            selected: false,
            range: range as f32,
            position,
            origin,
            callback
        }
    }

    fn position_is_valid(&self, memory: &components::MapMemory, map: &Map) -> bool {
        self.origin.distance(self.position) <= self.range && map.is_floor(self.position) && memory.is_visible(self.position.x, self.position.y)
    }
}

fn get_closest_entity_position(origin: Point, range: i32, state: &GameState) -> Option<Point> {
    use components::*;
    let ents = state.spatial_table.get_by_proximity(origin, range);
    let map_memory = state.spawning_pool.get::<MapMemory>(state.player)?;
    let entities: Vec<_> = ents.iter()
        .filter(|(pos, _)| map_memory.is_visible(pos.x, pos.y))
        .filter(|(_, id)| state.spawning_pool.get::<Stats>(*id).is_some())
        .map(|(_, id)| *id)
        .collect();
    if entities.len() > 1 {
        let physics = state.spawning_pool.get::<Physics>(entities[1])?;
        Some(physics.coord)
    } else {
        None
    }
}

impl Screen for SpellRayTargetScreen {
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
        let line = Line::new((self.origin.x, self.origin.y), (self.position.x, self.position.y));
        for (x, y) in line {
            tcod.add_animation(render::Animation::new(
                render::AnimationAnchor::Position{point: (x, y).into()},
                5.0, // time
                Some(10.0),
                vec![Some(('+', color))]
            ));
        }
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
            actions.push(Action {
                actor: Some(state.player),
                target: None,
                command: Command::Abort
            });
        }
        self.selected = false;
        ScreenResult::Stop
    }

    fn handle_input(&mut self, input: &Input, _state: &mut GameState) -> ScreenResult {
        match input.key {
            Key { code: KeyCode::Escape, .. } | Key { printable: 'q', .. } => {
                self.exit = true;
            },
            Key { code: KeyCode::Enter, .. } => {
                self.selected = true;
            },
            Key { code: KeyCode::Up, .. } | Key { printable: 'k', .. } => {
                self.position += (0, -1);
            },
            Key { printable: 'u', .. } => {
                self.position += (1, -1);
            },
            Key { code: KeyCode::Right, .. } | Key { printable: 'l', .. } => {
                self.position += (1, 0);
            },
            Key { printable: 'n', .. } => {
                self.position += (1, 1);
            },
            Key { code: KeyCode::Down, .. } | Key { printable: 'j', .. } => {
                self.position += (0, 1);
            },
            Key { printable: 'b', .. } => {
                self.position += (-1, 1);
            },
            Key { code: KeyCode::Left, .. } | Key { printable: 'h', .. } => {
                self.position += (-1, 0);
            },
            Key { printable: 'y', .. } => {
                self.position += (-1, -1);
            },
            _ => {}
        };
        ScreenResult::Stop
    }
}

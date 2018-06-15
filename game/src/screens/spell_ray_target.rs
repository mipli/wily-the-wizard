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
    position: Point,
    cursor: Point,
    selected: bool,
    origin: Point,
    range: f32,
    ray: Vec<(Point, bool)>,
    callback: Box<Fn(Point, &mut GameState, &mut Vec<Action>)>
}

impl SpellRayTargetScreen {
    pub fn new(origin: Point, range: i32, state: &GameState, callback: Box<Fn(Point, &mut GameState, &mut Vec<Action>)>) -> Self {
        let cursor = get_closest_entity_position(origin, range, state).unwrap_or(origin);
        SpellRayTargetScreen {
            exit: false,
            selected: false,
            range: range as f32,
            ray: vec![],
            position: cursor,
            cursor,
            origin,
            callback
        }
    }

    fn position_is_valid(&self, pos: Point, memory: &components::MapMemory, map: &Map) -> bool {
        self.origin.distance(pos) <= self.range && map.is_floor(pos) && memory.is_visible(pos.x, pos.y)
    }

    fn update_ray(&mut self, state: &mut GameState) {
        if let Some(memory) = state.spawning_pool.get::<components::MapMemory>(state.player) {
            let line = Line::new((self.origin.x, self.origin.y), (self.cursor.x, self.cursor.y));
            let mut valid = true;
            for (x, y) in line {
                valid = valid && self.position_is_valid((x, y).into(), memory, &state.map);
                if valid {
                    self.position = (x, y).into();
                }
                self.ray.push(((x, y).into(), valid));
            }
        }
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
        if self.selected {
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
            Key { code: KeyCode::Escape, .. } | Key { code: KeyCode::Text, printable: 'q', .. } => {
                self.exit = true;
            },
            Key { code: KeyCode::Enter, .. } => {
                self.selected = true;
            },
            Key { code: KeyCode::Up, .. } | Key { code: KeyCode::Text, printable: 'k', .. } => {
                self.cursor += (0, -1);
                self.ray.clear();
            },
            Key { code: KeyCode::Text, printable: 'u', .. } => {
                self.cursor += (1, -1);
                self.ray.clear();
            },
            Key { code: KeyCode::Right, .. } | Key { code: KeyCode::Text, printable: 'l', .. } => {
                self.cursor += (1, 0);
                self.ray.clear();
            },
            Key { code: KeyCode::Text, printable: 'n', .. } => {
                self.cursor += (1, 1);
                self.ray.clear();
            },
            Key { code: KeyCode::Down, .. } | Key { code: KeyCode::Text, printable: 'j', .. } => {
                self.cursor += (0, 1);
                self.ray.clear();
            },
            Key { code: KeyCode::Text, printable: 'b', .. } => {
                self.cursor += (-1, 1);
                self.ray.clear();
            },
            Key { code: KeyCode::Left, .. } | Key { code: KeyCode::Text, printable: 'h', .. } => {
                self.cursor += (-1, 0);
                self.ray.clear();
            },
            Key { code: KeyCode::Text, printable: 'y', .. } => {
                self.cursor += (-1, -1);
                self.ray.clear();
            },
            _ => {}
        };
        ScreenResult::Stop
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


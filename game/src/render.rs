use spawning_pool::{EntityId};
use tcod;
use tcod::console::*;
use tcod::colors;

use geo::*;
use consts::*;

use utils;
use game::*;
use components;

pub struct Tcod {
    pub root: Root,
    pub con: Offscreen,
    pub messages: Offscreen,
    pub stats_panel: Offscreen,
    pub info_panel: Offscreen,
    pub prev_time: f64,
    pub time: f64,
    pub animations: Vec<Animation>
}

impl Tcod {
    pub fn add_animation(&mut self, animation: Animation) {
        let mut animation = animation;
        animation.start = self.time;
        self.animations.push(animation);
    }
}

#[derive(Clone, Debug)]
pub enum AnimationAnchor {
    Entity{entity: EntityId},
    Position{point: Point}
}
#[derive(Clone)]
pub struct Animation {
    pub anchor: AnimationAnchor,
    pub time: f64,
    pub duration: Option<f64>,
    pub states: Vec<Option<(char, tcod::colors::Color)>>,
    pub state: usize,
    pub prev: f64,
    pub start: f64
}

impl Animation {
    pub fn new(anchor: AnimationAnchor, time: f64, duration: Option<f64>, states: Vec<Option<(char, tcod::colors::Color)>>) -> Self {
        Animation { 
            anchor,
            duration,
            time,
            states,
            state: 0 as usize,
            prev: 0.0,
            start: 0.0
        }
    }
    pub fn get_position(&self, game_state: &GameState) -> Option<Point> {
        match self.anchor {
            AnimationAnchor::Entity{entity} => utils::get_position(entity, &game_state.spawning_pool),
            AnimationAnchor::Position{point} => Some(point)
        }
    }
}

pub fn render(tcod: &mut Tcod, stats: components::Stats, game_state: &GameState, omnipotent: bool, delta: f64) -> Offscreen {
    tcod.time += delta;
    // aim to keep rendering speed at 60 fps
    /*
    if tcod.time < tcod.prev_time + 16.7 {
        return;
    }
    */
    let mut screen = Offscreen::new(tcod.root.width(), tcod.root.height());
    tcod.prev_time = tcod.time;
    render_map(&mut tcod.con, game_state, omnipotent);
    render_entities(&mut tcod.con, game_state, omnipotent);
    render_messages(&mut tcod.messages, game_state);
    render_stats_panel(&mut tcod.stats_panel, stats, game_state);
    render_info_panel(&mut tcod.info_panel, game_state);
    tcod.animations = render_animations(&mut tcod.con, &mut tcod.animations, game_state, tcod.time);

    blit(&tcod.con, (0, 0), (MAP_WIDTH, MAP_HEIGHT), &mut screen, (0, 0), 1.0, 1.0);
    blit(&tcod.messages, (0, 0), (MESSAGES_WIDTH, MESSAGES_HEIGHT), &mut screen, (0, MAP_HEIGHT), 1.0, 1.0);
    blit(&tcod.stats_panel, (0, 0), (STATS_PANEL_WIDTH, STATS_PANEL_HEIGHT), &mut screen, (MAP_WIDTH, 0), 1.0, 1.0);
    blit(&tcod.info_panel, (0, 0), (INFO_PANEL_WIDTH, INFO_PANEL_HEIGHT), &mut screen, (MAP_WIDTH, STATS_PANEL_HEIGHT), 1.0, 1.0);
    screen
}

fn render_animations(con: &mut Offscreen, animations: &mut Vec<Animation>, game_state: &GameState, time: f64) -> Vec<Animation>{
    let mut anims = vec![];

    while let Some(mut animation) = animations.pop() {
        animate(con, &mut animation, game_state, time);
        let cont = match animation.duration {
            Some(d) => animation.start + d > time,
            None => true
        };
        if cont {
            anims.push(animation);
        }
    }
    anims
}

fn animate(con: &mut Offscreen, animation: &mut Animation, game_state: &GameState, time: f64) {
    if let Some(pos) = animation.get_position(game_state) {
        if animation.time != 0.0 && time > animation.prev + animation.time {
            animation.prev = time;
            animation.state = (animation.state + 1) % animation.states.len() as usize;
        }
        if let Some((chr, color)) = animation.states[animation.state] {
            con.set_default_foreground(color);
            con.put_char(pos.x, pos.y, chr, BackgroundFlag::None);
        }
    }
}

fn render_messages(con: &mut Offscreen, game_state: &GameState) {
    con.set_default_background(colors::BLACK);
    con.set_default_foreground(colors::LIGHT_GREY);
    con.print_frame(0, 0, MESSAGES_WIDTH, MESSAGES_HEIGHT, true, BackgroundFlag::None, Some("Messages"));
    let mut y = MESSAGES_HEIGHT - 1;
    for &(ref msg, color) in game_state.messages.iter().rev() {
        let msg_height = con.get_height_rect(0, y - 1, MESSAGES_WIDTH - 2, 0, msg);
        y -= msg_height;
        con.set_default_foreground(color);
        con.print_rect(1, y, MESSAGES_WIDTH - 2, 0, msg);
        if y <= 1 {
            break;
        }
    }
}

fn render_entities(con: &mut Offscreen, game_state: &GameState, omnipotent: bool) {
    if let Some(memory) = game_state.spawning_pool.get::<components::MapMemory>(game_state.player) {
        let ids = game_state.spawning_pool.get_all::<components::Visual>();
        let mut to_draw: Vec<_> = ids.iter().filter(|d| {
            let &(id, _) = *d;
            if let Some(physics) = game_state.spawning_pool.get::<components::Physics>(id) {
                if omnipotent {
                    return true;
                }
                memory.is_visible(physics.coord.x, physics.coord.y)
            } else {
                return false;
            }
        }).map(|&(id, _)| {
            let visual = game_state.spawning_pool.get::<components::Visual>(id).unwrap();
            let physics = game_state.spawning_pool.get::<components::Physics>(id).unwrap();
            let flags = game_state.spawning_pool.get::<components::Flags>(id).unwrap();
            (physics, visual, flags)
        }).collect();
        to_draw.sort_by(|a, b| {
            (*a).2.solid.cmp(&b.2.solid)
        });
        for draw in to_draw {
            con.set_default_foreground(draw.1.color);
            con.put_char(draw.0.coord.x, draw.0.coord.y, draw.1.glyph, BackgroundFlag::None);
        }
    }
}

fn render_map(con: &mut Offscreen, game_state: &GameState, omnipotent: bool) {
    con.clear();
    if let Some(memory) = game_state.spawning_pool.get::<components::MapMemory>(game_state.player) {
        for x in 0..game_state.map.dimensions.x {
            for y in 0..game_state.map.dimensions.y {
                let cell = game_state.map.get_cell(x, y);

                let in_view = omnipotent || memory.is_visible(x, y);
                let explored = memory.is_explored(x, y);

                if in_view || explored {
                    let (glyph, color, background_color) = cell.get_render_info(in_view);
                    con.set_default_foreground(color);
                    con.put_char(x, y, glyph, BackgroundFlag::None);
                    con.set_char_background(x, y, background_color, BackgroundFlag::Set);
                }
            }
        }
    }
}

fn render_info_panel(panel: &mut Offscreen, game_state: &GameState) {
    panel.set_default_background(colors::BLACK);
    panel.set_default_foreground(colors::LIGHT_GREY);
    panel.print_frame(0, 0, INFO_PANEL_WIDTH, INFO_PANEL_HEIGHT, true, BackgroundFlag::None, Some("Information"));

    panel.print_rect(4, INFO_PANEL_HEIGHT-1, INFO_PANEL_WIDTH - 2, 0, format!("Tick: {}", game_state.scheduler.time));

    if let Some(memory) = game_state.spawning_pool.get::<components::MapMemory>(game_state.player) {
        if let Some(physics) = game_state.spawning_pool.get::<components::Physics>(game_state.player) {
            panel.print_rect(1, INFO_PANEL_HEIGHT-2, INFO_PANEL_WIDTH - 2, 0, format!("Coord: {}", physics.coord));
            let entities = game_state.spatial_table.in_circle(physics.coord, 5);
            panel.set_default_foreground(colors::WHITE);
            let mut y = 2;
            for (pos, entity) in entities {
                if !memory.is_visible(pos.x, pos.y) || entity == game_state.player {
                    continue;
                }
                let msg = utils::describe_entity(entity, &game_state.spawning_pool);
                let msg_height = panel.get_height_rect(0, y - 1, INFO_PANEL_WIDTH - 2, 0, &msg);
                panel.print_rect(1, y, INFO_PANEL_WIDTH - 2, 0, msg);
                y += msg_height;
            }
        }
    }
}

fn render_stats_panel(panel: &mut Offscreen, stats: components::Stats, game_state: &GameState) {
    panel.set_default_background(colors::BLACK);
    panel.set_default_foreground(colors::LIGHT_GREY);
    panel.print_frame(0, 0, STATS_PANEL_WIDTH, STATS_PANEL_HEIGHT, true, BackgroundFlag::None, Some("Stats"));

    let hp_bar = Bar {
        name: "HP".to_owned(),
        total_width: 13,
        value: stats.health,
        max_value: stats.max_health
    };
    render_bar(panel, (1, 4).into(), &hp_bar, colors::LIGHT_RED, colors::DARKER_RED);

    panel.set_default_foreground(colors::WHITE);
    panel.print_ex(
        1,
        6,
        BackgroundFlag::None,
        TextAlignment::Left,
        &format!("Strength: {}", stats.strength + utils::get_strength_bonus(game_state.player, &game_state.spawning_pool))
    );
    panel.print_ex(
        1,
        7,
        BackgroundFlag::None,
        TextAlignment::Left,
        &format!("Defense:  {}", stats.defense + utils::get_defense_bonus(game_state.player, &game_state.spawning_pool))
    );
}

struct Bar {
    name: String,
    total_width: i32,
    value: i32,
    max_value: i32
}
 
#[cfg_attr(feature = "cargo-clippy", allow(blacklisted_name))]
fn render_bar(
    panel: &mut Offscreen,
    pos: Point,
    bar: &Bar,
    bar_color: tcod::Color,
    background_color: tcod::Color,
) {
    let bar_width = ((bar.value as f32 / bar.max_value as f32) * bar.total_width as f32) as i32;

    panel.set_default_background(background_color);
    panel.rect(pos.x, pos.y, bar.total_width, 1, false, BackgroundFlag::Set);

    if bar_width > 0 {
        panel.set_default_background(bar_color);
        panel.rect(
            pos.x + bar.total_width - bar_width,
            pos.y,
            bar_width,
            1,
            false,
            BackgroundFlag::Screen,
        );
    }

    panel.set_default_foreground(colors::WHITE);
    panel.print_ex(
        pos.x + bar.total_width / 2,
        pos.y,
        BackgroundFlag::None,
        TextAlignment::Center,
        &format!("{}: {}/{}", bar.name, bar.value, bar.max_value),
    );
}

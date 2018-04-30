 #![feature(drain_filter)]

extern crate time;
extern crate yaml_rust;
extern crate tcod;
extern crate rand;
extern crate inflector;

extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

use tcod::console::*;

#[macro_use] extern crate spawning_pool;

extern crate map_generator;
extern crate geo;

mod messages;
mod spells;
mod screens;
mod save;
mod path;
mod utils;
mod consts;
mod spatial;
mod map;
mod scheduler;
mod systems;
mod components;
mod render;
mod game;
mod ai;
mod actions;
mod rules;
mod creatures;

use game::*;
use geo::*;
use consts::*;
use render::*;
use map::*;
use spatial::*;
use actions::*;

fn main() {
    let root = Root::initializer()
        .font("fonts/lucida12x12_gs_tc.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("SNEAKY!")
        .init();

    let mut tcod = Tcod {
        root,
        con: Offscreen::new(MAP_WIDTH, MAP_HEIGHT),
        messages: Offscreen::new(MESSAGES_WIDTH, MESSAGES_HEIGHT),
        stats_panel: Offscreen::new(STATS_PANEL_WIDTH, STATS_PANEL_HEIGHT),
        info_panel: Offscreen::new(INFO_PANEL_WIDTH, INFO_PANEL_HEIGHT),
        prev_time: 0.0,
        time: 0.0,
        animations: vec![]
    };

    run_game(&mut tcod);
}

/*
*/


fn run_game(tcod: &mut Tcod) {
    let mut manager = screens::ScreenManager::new();
    let mut game = screens::create_new_game();

    manager.add_screen(Box::new(screens::main_menu::MainMenuScreen::new()));
    let mut t_0 = time::precise_time_ns();
    let mut t_1;
    let mut t_delta: f64;
    while !tcod.root.window_closed() && !manager.screens.is_empty() {
        t_1 = time::precise_time_ns();
        t_delta = (t_1 - t_0) as f64 / 1_000_000.0;
        t_0 = t_1;
        // let fps = 1000.0 / t_delta;
        // println!("fps: {}", fps);
        //
        let mut actions = vec![];

        manager.handle_input(&mut game.state);
        manager.tick(&mut game.state, tcod, &mut actions);

        if actions.iter().any(|a| a.command == Command::CreateGame) {
            game = screens::create_new_game();
        }
        if actions.iter().any(|a| a.command == Command::LoadGame) {
            game.state = screens::main_menu::load_game();
            game.state.spatial_table.dirty = true;
        }

        if game.state.spatial_table.dirty {
            game.state.spatial_table.dirty = false;
            update_fov_map(&mut game.fov, &game.state.map, &game.state.spatial_table);
        }

        let mut animations = vec![];
        let tick_result = if game.current_action.is_none() || !actions.is_empty() {
            game.game_tick(actions, &mut animations)
        } else {
            TickResult::Wait(WaitResult::Wait)
        };

        for animation in animations {
            tcod.add_animation(animation);
        }

        manager.render(t_delta, &mut game.state, &game.fov, tcod);

        match tick_result {
            TickResult::Passed => {
                game.state.spawning_pool.cleanup_removed();
            },
            TickResult::Wait(WaitResult::Wait) => {},
            TickResult::Wait(WaitResult::RequireTarget{action}) => {
                if let Some(physics) = game.state.spawning_pool.get::<components::Physics>(game.state.player) {
                    manager.add_screen(Box::new(screens::SingleTargetScreen::new(
                        physics.coord,
                        &game.state,
                        Box::new(move |e, _state, actions| {
                            let mut act = action.clone();
                            act.target = Some(e);
                            actions.push(act);
                        })
                    )));
                }
            }
        }

        manager.add_screens(&mut game.state);
        manager.clear_screens(&mut game.state);
    }
}

pub fn update_fov_map(fov: &mut tcod::map::Map, map: &Map, spatial_table: &SpatialTable) {
    for x in 0..spatial_table.width {
        for y in 0..spatial_table.height {
            let (mut blocks_sight, mut blocks_movement) = (map.get_cell(x, y).blocks_sight, map.get_cell(x, y).blocks_movement);
            if let Some(spatial_cell) = spatial_table.get(Point::new(x, y)) {
                if spatial_cell.solid {
                    blocks_movement = true;
                }
                if spatial_cell.opaque {
                    blocks_sight = true;
                }
            }
            fov.set(x, y, !blocks_sight, !blocks_movement);
        }
    }
}


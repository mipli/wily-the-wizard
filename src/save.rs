use serde_json;

use std::io::{self, Read, Write};
use std::fs::File;

use game::*;

pub fn save_game(game_state: &GameState) -> Result<(), io::Error> {
    let save_data = serde_json::to_string(&game_state).unwrap();
    let mut file = File::create("savegame")?;
    file.write_all(save_data.as_bytes())?;
    Ok(())
}

pub fn load_game() -> Result<GameState, io::Error> {
    let mut data = String::new();
    let mut file = File::open("savegame")?;
    file.read_to_string(&mut data)?;
    let state: GameState = serde_json::from_str(&data)?;
    Ok(state)

}

mod gamedata;
use gamedata::*;

use serde::{Serialize, Deserialize};
use std::sync::Mutex;

#[derive(Debug, Serialize)]
pub struct RootResponse {
    apiversion: String,
    author: String,
    color: String,
    head: String,
    tail: String,
}
impl RootResponse {
    pub fn new<'a>(apiversion: &'a str, author: &'a str, color: &'a str, head: &'a str, tail: &'a str) -> RootResponse {
        RootResponse {apiversion: String::from(apiversion), author: String::from(author), color: String::from(color), head: String::from(head), tail: String::from(tail)}
    }
    pub fn default() -> RootResponse {RootResponse::new("1", "shantanuvichare", "#7303fc", "shac-gamer", "shac-mouse")}
}

#[derive(Debug, Serialize)]
pub struct MoveResponse {
    #[serde(rename(serialize = "move"))]
    _move: String,
    shout: String
}
impl MoveResponse {
    pub fn new<'a>(_move: &'a str, shout: &'a str) -> MoveResponse {
        MoveResponse {_move: String::from(_move), shout: String::from(shout)}
    }

}


#[derive(Debug, Deserialize)]
pub struct RequestBody {
    game: Game,
    turn: usize,
    board: Board,
    you: Battlesnake,
}

pub struct AppStateWrapper {
    state: Mutex<AppState>,
}
impl AppStateWrapper {
    pub fn new() -> AppStateWrapper {
        AppStateWrapper {
            state: Mutex::new(AppState::new()),
        }
    }
    pub fn initialise(&self, body: &RequestBody) {
        self.state.lock().unwrap().initialise(&body.game, body.turn, &body.board, &body.you);
    }
    pub fn update(&self, body: &RequestBody) {
        self.state.lock().unwrap().update(&body.game, body.turn, &body.board, &body.you);
    }
    pub fn get_response(&self) -> String {
        self.state.lock().unwrap().get_response()
    }
    pub fn end_game(&self, _body: &RequestBody) {
        self.state.lock().unwrap().renew();
    }
}


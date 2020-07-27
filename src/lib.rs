use serde::{Serialize, Deserialize};
use std::{borrow::Borrow, sync::Mutex};

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

#[derive(Debug, Deserialize)]
struct Game {
    id: String,
    timeout: usize,
}

#[derive(Debug, Deserialize)]
pub struct Board {
    height: usize,
    width: usize, 
    food: Vec<Point>,
    snakes: Vec<Battlesnake>,
}

#[derive(Debug, Deserialize)]
struct Battlesnake {
    id: String,
    name: String,
    health: usize,
    body: Vec<Point>,
    head: Point,
    length: usize,
    shout: String,
}

// Auxiliaries
#[derive(Debug, Deserialize)]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Deserialize)]
enum Direction { no_idea, up, down, left, right }

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
        self.state.lock().unwrap().initialise(body);
    }
    pub fn update(&self, body: &RequestBody) {
        self.state.lock().unwrap().update(body);
    }
    pub fn get_response(&self) -> &str {
        match self.state.lock().unwrap().next_move {
            Direction::up => "up",
            Direction::down => "down",
            Direction::left => "left",
            Direction::right => "right",
            Direction::no_idea => {
                eprintln!("Response was asked without updating 'next_move' !");
                "up"
            }
        }
    }
    pub fn end_game(&self, body: &RequestBody) {
        self.state.lock().unwrap().renew();
    }
}

struct AppState {
    next_move: Direction,
}

impl AppState {
    fn new() -> AppState {
        AppState {
            next_move: Direction::no_idea,
        }
        // Remember to copy this config to renew method
    }
    fn renew(&mut self) {
        self.next_move = Direction::no_idea;
    }
    fn initialise(&mut self, body: &RequestBody) {

    }
    fn update(&mut self, body: &RequestBody) {

    }
    
}

use serde::{Serialize, Deserialize};

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
impl RequestBody {
    // pub fn get_response(&self) -> Direction {
    //     Direction::left
    // }
}

#[derive(Debug, Deserialize)]
struct Game {
    id: String,
    timeout: usize,
}

#[derive(Debug, Deserialize)]
pub struct Board {
    // height: usize,
    // width: usize, 
    pub food: Vec<Point>,
    // snakes: Vec<Battlesnake>,
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
enum Direction { up, down, left, right }

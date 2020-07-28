use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Game {
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
pub struct Battlesnake {
    id: String,
    name: String,
    health: usize,
    body: Vec<Point>,
    head: Point,
    length: usize,
    shout: String,
}

enum Direction { NoIdea, Up, Down, Left, Right }

#[derive(Debug, Deserialize)]
struct Point {
    x: usize,
    y: usize,
}




pub struct AppState {
    next_move: Direction,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            next_move: Direction::NoIdea,
        }
        // Remember to copy this config to "renew" method
    }
    pub fn renew(&mut self) {
        self.next_move = Direction::NoIdea;
    }
    pub fn initialise(&mut self, game: &Game, turn: usize, board: &Board, you: &Battlesnake) {

    }
    pub fn update(&mut self, game: &Game, turn: usize, board: &Board, you: &Battlesnake) {

    }
    pub fn get_response(&self) -> String {
        String::from( match self.next_move {
            Direction::Up => "up",
            Direction::Down => "down",
            Direction::Left => "left",
            Direction::Right => "right",
            Direction::NoIdea => {
                eprintln!("Response was asked without updating 'next_move' !");
                "up"
            }
        })
    }
}


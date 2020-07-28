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
impl PartialEq for Battlesnake {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction { NoIdea, Up, Down, Left, Right }

#[derive(Debug, Copy, Clone, PartialEq, Deserialize)]
struct Point {
    x: usize,
    y: usize,
}
impl Point {
    fn manhattan(&self, other: &Self) -> i32 {
        let x = self.x as i32 - other.x as i32;
        let y = self.y as i32 - other.y as i32;
        x.abs() + y.abs()
    }
}


pub struct AppState {
    next_move: Direction,
    data: Option<DirectedPoints>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            next_move: Direction::NoIdea,
            data: None,
        }
        // Remember to copy this config to "renew" method
    }
    pub fn renew(&mut self) {
        self.next_move = Direction::NoIdea;
        self.data = None;
    }
    pub fn initialise(&mut self, game: &Game, turn: usize, board: &Board, you: &Battlesnake) {
        let data = DirectedPoints::new(you, true);
        self.data = Some(data);
        println!("Data initialised as expected");
        self.update(game, turn, board, you);
    }
    pub fn update(&mut self, game: &Game, turn: usize, board: &Board, you: &Battlesnake) {

        let data = match self.data.as_mut() {
            Some(data) => data,
            None => {
                let data = DirectedPoints::new(you, true);
                self.data = Some(data);
                println!("Data initialised during Update");
                self.data.as_mut().unwrap()
            },
        };

        println!("Snake: {}\nGame ID: {}\nTurn: {}\n", &you.name, &game.id, turn);

        data.clear_border_points(board.height, board.width);
        for snake in board.snakes.iter() {
            data.add_snake_body(snake);
            data.evaluate_snake(snake, -1*(board.height+board.width)as i32);
        }
        data.evaluate_food(&board.snakes, &board.food, (board.height+board.width)as i32);
        self.next_move = data.get_next_move();
    }
    pub fn get_response(&self) -> String {
        String::from( match self.next_move {
            Direction::Up => "up",
            Direction::Down => "down",
            Direction::Left => "left",
            Direction::Right => "right",
            _ => {
                eprintln!("Response was asked without updating 'next_move' !");
                "up"
            }
        })
    }
}

#[derive(Debug)]
struct DirectedPoints { // Maps to (H+2)x(W+2) sized grid
    my_id: String,
    dirs: Vec<(Direction, Point, Option<i32>)>,
    debug_flag: bool,
}
impl DirectedPoints {
    fn new(you: &Battlesnake, debug_flag: bool) -> DirectedPoints {
        let (map_x, map_y) = (you.head.x+1, you.head.y+1);
        let dirs = vec![
            (Direction::Left, Point {x: map_x -1, y: map_y}, Some(0)),
            (Direction::Up , Point {x: map_x, y: map_y +1}, Some(0)),
            (Direction::Right, Point {x: map_x +1, y: map_y}, Some(0)),
            (Direction::Down, Point {x: map_x, y: map_y -1}, Some(0)),
        ];
        if debug_flag { println!("Dirs at init:{:?}", dirs) }
        DirectedPoints {my_id: you.id.clone(), dirs, debug_flag}
    }
    fn clear_border_points(&mut self, height: usize, width: usize) {
        for (_d, Point{x,y}, val) in self.dirs.iter_mut() {
            if (*x==0)|(*x==width+1)|(*y==0)|(*y==height+1) { *val = None }
        }
        if self.debug_flag { println!("Dirs after clearing border points:{:?}", self.dirs) }
    }
    fn add_snake_body(&mut self, snake: &Battlesnake) {
        for (_d, p, val) in self.dirs.iter_mut() {
            if snake.body.contains(p) { *val = None }
        }
        if self.debug_flag { println!("Dirs after adding Snake body:{:?}", self.dirs) }
    }
    fn evaluate_snake(&mut self, snake: &Battlesnake, extreme_val: i32) {
        for (_d, p, val) in self.dirs.iter_mut() {
            if let Some(value) = val { *value += extreme_val+p.manhattan(&snake.head) }
        }
        if self.debug_flag { println!("Dirs after evaluating Snake:{:?}", self.dirs) }
    }
    fn evaluate_food(&mut self, snakes: &Vec<Battlesnake>, food: &Vec<Point>, extreme_val: i32) {
        let viable_food = food.iter().map(|f| {
            snakes.iter().map(|s| (f, s, s.head.manhattan(f)))
            .min_by_key(|(_f, _s,d)| *d).unwrap()
        }).filter(|(_f,s,_d)| s.id == self.my_id)
        .min_by_key(|(_f,_s,d)| *d);
        
        if let Some((f,_s,_d)) = viable_food {
            for (_d, p, val) in self.dirs.iter_mut() {
                if let Some(value) = val { *value += extreme_val+p.manhattan(f) }
            }
        }
        if self.debug_flag { println!("Dirs after evaluating Food:{:?}", self.dirs) }
    }
    fn get_next_move(&self) -> Direction {
        if self.debug_flag { println!("Dirs before deciding next move:{:?}", self.dirs) }
        let mut dir = Direction::NoIdea;
        let mut max_val = i32::MIN;
        for (d, _p, val) in self.dirs.iter() {
            if let Some(value) = val { 
                if *value >= max_val {dir = *d; max_val = *value;}
            }
        }
        dir
    }
    // fn inside_grid(x: usize,y: usize,width: usize,height: usize) -> bool {
    //     (x==0)|(x==width+1)|(y==0)|(y==height+1)
    // }
}

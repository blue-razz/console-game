use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::{color, style};
use std::io::{Write, stdout, stdin};
use rand::distributions::Uniform;
use rand::prelude::*;
use std::time::Duration;
use std::collections::HashMap;

struct Point {
    x: i32,
    y: i32,
}

struct Map {
    width: i32,
    height: i32,
    map: Vec<Vec<char>>
}

struct Player {
    id: i32,
    health: i32,
    position: Point,
}

struct GameState {
    map: Map,
    player_map: HashMap<i32, Point>,
}

impl Map {
    fn new(width: i32, height: i32) -> Map {
        Map {
            width,
            height,
            map: vec![vec!['.'; width as usize]; height as usize]
        }
    }

    fn generate_trees(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                    if self.map[y as usize][x as usize] == '.' {
                        let mut rng = rand::thread_rng();
                        let tree_chance = Uniform::new(0, 100);
                        let tree_roll = tree_chance.sample(&mut rng);

                        if tree_roll < 5 {
                            self.map[y as usize][x as usize] = 'T';
                        }
                    }
                    
            }
        }
    }

    fn generate_rivers(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.map[y as usize][x as usize] == '.' { 
                    let mut rng = rand::thread_rng();
                    let river_chance = Uniform::new(0, 1000);
                    let river_roll = river_chance.sample(&mut rng);

                    if river_roll == 885 {
                        self.map[y as usize][x as usize] = 'R';
                    }
                }
            }
        }
    }

    fn generate_mountains(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.map[y as usize][x as usize] == '.' { 
                    let mut rng = rand::thread_rng();
                    let mountain_chance = Uniform::new(0, 1000);

                        // Is this the upper bound?
                    if y < 6 {
                        let mountain_roll = mountain_chance.sample(&mut rng);

                        if mountain_roll < 500 {
                            self.map[y as usize][x as usize] = 'M';
                        }
                    } else if x < 8 {
                        let mountain_roll = mountain_chance.sample(&mut rng);

                        if mountain_roll < 500 {
                            self.map[y as usize][x as usize] = 'M';
                        }
                    } else if y > self.height - 6 {
                        let mountain_roll = mountain_chance.sample(&mut rng);

                        if mountain_roll < 500 {
                            self.map[y as usize][x as usize] = 'M';
                        }
                    } else if x > self.width - 8 {
                        let mountain_roll = mountain_chance.sample(&mut rng);

                        if mountain_roll < 500 {
                            self.map[y as usize][x as usize] = 'M';
                        }
                    } else {
                        let mountain_roll = mountain_chance.sample(&mut rng);

                        if mountain_roll == 420 {
                            self.map[y as usize][x as usize] = 'M';
                        }
                    }
                }
            }
        }
    }

    fn generate_map(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                // This is the border
                if y == 0 || y == self.height - 1 || x == 0 || x == self.width - 1 {
                    self.map[y as usize][x as usize] = '#';
                } else {
                    self.map[y as usize][x as usize] = '.';
                }
            }
        }

        self.generate_trees();
        self.generate_rivers();
        self.generate_mountains();
    }
}

impl Player {
    fn new(id: i32) -> Player {
        Player {
            id,
            health: 100,
            position: Point {
                x: 32,
                y: 32
            }
        }
    }
}

fn write_intro_screen<W: Write>(screen: &mut W) {
    let (term_width, term_height) = termion::terminal_size().unwrap();

    write!(screen, "{}{}{}Welcome to console game!{}", termion::cursor::Goto(term_width / 2 - 12, term_height / 2 - 2), color::Fg(color::White), style::Bold, style::Reset).unwrap();
    write!(screen, "{}{}x{}", termion::cursor::Goto(term_width / 2, term_height / 2 - 1), term_width, term_height).unwrap();
    write!(screen, "{}{}{}{}", termion::cursor::Goto(term_width / 2 - 12, term_height / 2 + 3), style::Blink, "Press any key to continue...", style::Reset).unwrap();

}

fn render_player<W: Write>(screen: &mut W, game_state: &GameState) {
    for p in game_state.player_map.values() {
        write!(screen, "{}{}{}{}@{}", termion::cursor::Goto(p.x as u16, p.y as u16), color::Fg(color::Magenta), style::Bold, style::Blink, style::Reset ).unwrap();

        
        screen.flush().unwrap();
    }
}

fn render<W: Write>(screen: &mut W, game_state: &GameState) {
    write!(screen, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
    for y in 0..game_state.map.height {
        for x in 0..game_state.map.width {
            match game_state.map.map[y as usize][x as usize] {
                'M' => write!(screen, "{}M{}", color::Fg(color::Rgb(128, 128, 128)), style::Reset).unwrap(),
                'R' => write!(screen, "{}|", color::Fg(color::Blue)).unwrap(),
                '#' => write!(screen, "{}#", color::Fg(color::White)).unwrap(),
                'T' => write!(screen, "{}T", color::Fg(color::Green)).unwrap(),
                _ => write!(screen, "{} ", color::Fg(color::Reset)).unwrap(),
            }


        }
        write!(screen, "{}", termion::cursor::Goto(1, (y + 2) as u16)).unwrap();
        screen.flush().unwrap();
    }

}

fn main() {
    let (term_width, term_height) = termion::terminal_size().unwrap();

    let stdin = stdin();
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    
    write_intro_screen(&mut screen);

    screen.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            _ => break
        }
    }
    

    let mut state = GameState {
        map: Map::new(term_width as i32, term_height as i32),
        player_map: HashMap::new()
    };

    state.map.generate_map();
    let p1 = Player::new(1);

    state.player_map.insert(p1.id, p1.position);
    

    'game_loop: loop {
        let stdin = std::io::stdin();
        
        
        render_player(&mut screen, &state);
        render(&mut screen, &state);
        

        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char('q') => break 'game_loop,
                Key::Char('w') => {
                    let mut p = state.player_map.get_mut(&1).unwrap();
                    p.y -= 1;
                    write!(screen, "{}{}", termion::cursor::Goto(p.x as u16, p.y as u16), state.map.map[p.y as usize][p.x as usize]).unwrap();
                    render_player(&mut screen, &state)
                },
                Key::Char('a') => {
                    let mut p = state.player_map.get_mut(&1).unwrap();
                    p.x -= 1;
                    render_player(&mut screen, &state)
                },
                Key::Char('s') => {
                    let mut p = state.player_map.get_mut(&1).unwrap();
                    p.y += 1;
                    render_player(&mut screen, &state)
                },
                Key::Char('d') => {
                    let mut p = state.player_map.get_mut(&1).unwrap();
                    p.x += 1;
                    render_player(&mut screen, &state)
                },
                _ => {}
            }

            screen.flush().unwrap();
        }

        

        std::thread::sleep(Duration::from_millis(17));
    }
}

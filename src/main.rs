use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::{color, style};
use std::io::{Write, stdout, stdin};
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
}

struct GameState {
    map: Map,
    player_map: HashMap<Player, Point>,
}

impl Map {
    fn new(width: i32, height: i32) -> Map {
        Map {
            width,
            height,
            map: vec![vec![' '; width as usize]; height as usize]
        }
    }
}

impl Player {
    fn new(id: i32) -> Player {
        Player {
            id,
            health: 100
        }
    }
}

fn write_alt_screen<W: Write>(screen: &mut W) {
    write!(screen, "{}{}Welcome to the alternate screen!{}Second line!", 
        termion::clear::All, termion::cursor::Goto(1, 1), termion::cursor::Goto(1, 2)).unwrap();
}

fn main() {
    let (term_width, term_height) = termion::terminal_size().unwrap();
    let stdin = stdin();
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    write!(screen, "{}{}{}Welcome to my console game!{}", termion::cursor::Goto(term_width / 2 - 10, term_height / 2 - 2), color::Fg(color::Magenta), style::Bold, style::Reset).unwrap();
    write!(screen, "{}{}x{}", termion::cursor::Goto(term_width / 2, term_height / 2 - 1), term_width, term_height).unwrap();

    screen.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            _ => {}
        }

        screen.flush().unwrap();
    }
}

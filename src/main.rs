use termion::event::Key;
use termion::input::TermRead;
use termion::{color, style};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::*;
use std::io::{Write, stdout, stdin};

struct Coord {
    pub x: u16,
    pub y: u16,
}

struct Player {
    pos: Coord,
}

struct Screen<'a, W: Write> {
    canvas: &'a mut W,
    width: u16,
    height: u16,
}

struct Game<'a, W: Write> {
    screen: &'a mut Screen<'a, W>,
    player: &'a mut Player
}

impl<W: Write> Game<'_, W> {
    pub fn move_player(&mut self, dir: Key) {
        match dir {
            Key::Char('w') => {
                if self.player.pos.y > 1 {
                    self.player.pos.y -= 1;
                }
            },
            Key::Char('s') => {
                if self.player.pos.y < self.screen.height {
                    self.player.pos.y += 1;
                }
            },
            Key::Char('a') => {
                if self.player.pos.x > 1 {
                    self.player.pos.x -= 1;
                }
            },
            Key::Char('d') => {
                if self.player.pos.x < self.screen.width {
                    self.player.pos.x += 1;
                }
            },
            _ => {
                return;
            }
        }
    }
}

fn write_intro_screen<W: Write>(screen: &mut W) {
    let (width, height) = termion::terminal_size().unwrap();
    let mut center_x = width / 2;
    let center_y = height / 2;

    if width > 150 {
        center_x -= 4;
    } 

    if width > 180 {
        center_x -= 8;
    }

    write!(screen, "{}{}{}Welcome to CONSOLE GAME!! ({}x{}){}",
        termion::clear::All,
        termion::cursor::Goto(center_x - 8, center_y - 5),
        style::Bold,
        width, height,
        termion::cursor::Goto(center_x - 8, center_y - 2),
    ).unwrap();

    write!(screen, "{}1. Start Game{}{}2. Load Game{}{}3. Exit{}{}",
        color::Fg(color::Magenta),
        termion::cursor::Goto(center_x - 8, center_y - 1),
        color::Fg(color::Green),
        termion::cursor::Goto(center_x - 8, center_y),
        color::Fg(color::Red),
        style::Reset,
        termion::cursor::Goto(center_x - 8, center_y + 1),
    ).unwrap();
}

fn write_game_screen<W: Write>(game: &mut Game<W>) {

    write!(game.screen.canvas, "{}{}P",
        termion::clear::All,
        termion::cursor::Goto(game.player.pos.x, game.player.pos.y),
    ).unwrap();

    game.screen.canvas.flush().unwrap();

}

fn main() {
    // Setup termion
    let mut game_screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    write!(game_screen, "{}", termion::cursor::Hide).unwrap();

    write_intro_screen(&mut game_screen);
    game_screen.flush().unwrap();

    // Setup game
    
    

    'intro_loop: loop {
        let stdin = stdin();

        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char('1') => {
                    break 'intro_loop;
                },
                Key::Char('2') => {
                    break 'intro_loop;
                },
                Key::Char('3') => {
                    panic!("Exiting...");
                },
                _ => {},
            }
        }
    }

    'game_loop: loop {
        let (width, height) = termion::terminal_size().unwrap();
        
        let mut p1 = Player {
            pos: Coord {
                x: 1,
                y: 5,
            },
        };
        
        let mut game = Game {
            screen: &mut Screen {
                canvas: &mut game_screen,
                width,
                height,
            },
            player: &mut p1,
        };

        

        let stdin = std::io::stdin();

        'keys: for c in stdin.keys() {
            match c.unwrap() {
                Key::Char('q') => {
                    break 'game_loop;
                },           
                x => {game.move_player(x)}
            }

            write_game_screen(&mut game);
        }
    }
}

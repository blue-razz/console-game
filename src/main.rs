use std::io;
use std::io::Write;

struct Player {
    pos: Vec<i32>,
}

struct World {
    players: Vec<Player>,
    height: i32,
    width: i32,
    map: Vec<Vec<char>>,
}

impl World {
    fn new(height: i32, width: i32) -> World {
        World {
            players: Vec::new(),
            height: height,
            width: width,
            map: vec![vec!['.'; width as usize]; height as usize],
        }
    }
    fn generate_world(&mut self) {
        for i in 0..self.height {
            for j in 0..self.width {
                    // Is this the first row?
                if i == 0 {
                    self.map[i as usize][j as usize] = 'M';
                }

                    // Is this the last row?
                else if i == self.height - 1 {
                    self.map[i as usize][j as usize] = 'M';
                }

                    // Is this the first column?
                else if j == 0 {
                    self.map[i as usize][j as usize] = 'M';
                }

                    // Is this the last column?
                else if j == self.width - 1 {
                    self.map[i as usize][j as usize] = 'M';
                }

                    // Is this a corner?
                else if i == 0 && j == 0 {
                    self.map[i as usize][j as usize] = 'M';
                }
            }
        }
    }

    fn spawn_player(&mut self) {
        self.players.push(Player { pos: Vec::from([&self.height/2, &self.width/2])});
    }

    fn move_player_up(&mut self) {
        if self.is_blocked(self.players[0].pos[0] - 1, self.players[0].pos[1]) {
            return;
        }

        self.players[0].pos[0] -= 1;
    }
    fn move_player_down(&mut self) {
        if self.is_blocked(self.players[0].pos[0] + 1, self.players[0].pos[1]) {
            return;
        }

        self.players[0].pos[0] += 1;
    }
    fn move_player_left(&mut self) {
        if self.is_blocked(self.players[0].pos[0], self.players[0].pos[1] - 1) {
            return;
        }
        self.players[0].pos[1] -= 1;
    }
    fn move_player_right(&mut self) {
        if self.is_blocked(self.players[0].pos[0], self.players[0].pos[1] + 1) {
            return;
        }
        self.players[0].pos[1] += 1;
    }

    fn is_blocked(&self, y: i32, x: i32) -> bool {
        if x < 0 || y >= self.height || y < 0 || x >= self.width {
            return true;
        }
        if self.map[y as usize][x as usize] == 'M' {
            return true;
        }
        return false;
    }

    
}

fn tile_has_player(position: Vec<i32>, world: &World) -> bool {
    for p in &world.players {
        let is_on_tile = p.pos.iter()
                            .zip(position.iter()).filter(|(p, t)| p == t).count() == 2;

        if is_on_tile { return true; }
    }

    false
}

fn print_world(world: &World) {
    for i in 0..world.height {
        for j in 0..world.width {
            if tile_has_player(Vec::from([i, j]), world) {
                print!("P");
            } else {
                print!("{}", world.map[i as usize][j as usize]);
            }
        }
        println!("");
    }


    println!();
}

fn main() {
    let mut world: World = World::new(40, 120);
    world.generate_world();
    world.spawn_player();

    'game_loop: loop {
        print!("\x1B[2J\x1B[1;1H");
        println!("[{}, {}]", world.players[0].pos[0], world.players[0].pos[1]);
        print_world(&world);

        print!("Enter command: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        match input {
            "move up" => world.move_player_up(),
            "move down" => world.move_player_down(),
            "move left" => world.move_player_left(),
            "move right" => world.move_player_right(),
            "quit" => break 'game_loop,
            _ => println!("Unknown command: {}", input),
        }
    }
    
}

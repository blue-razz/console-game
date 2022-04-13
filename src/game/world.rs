use super::player::Player;
use std::io;
use std::io::Write;

struct Position(i32, i32);

pub struct World {
    world_name: String,
    pub height: i32,
    pub width: i32,
    pub map: Vec<Vec<char>>,
    player_list: Vec<Player>
}

impl World {
    pub fn new(world_name: String, height: i32, width: i32) -> World {
        World {
            world_name,
            height,
            width,
            player_list: vec![],
            map: vec![vec!['.'; width as usize]; height as usize]
        }
    }

    pub fn spawn_player(&mut self, player_name: String) {
        self.player_list.push(Player::new(player_name, vec![16, 40]));
    }

    // Generate the world given the world's height and width
    pub fn generate_world(&mut self) {
        let MAX_Y: i32 = self.height - 1;
        let MAX_X: i32 = self.width - 1;
        
        // Generate axis
        for y in 0..self.height {
            for x in 0..self.width { 
                // Is this the first row?
                if y == 0 {
                    self.map[y as usize][x as usize] = 'M';
                }

                // Is this the last row?
                else if y == MAX_Y {
                    self.map[y as usize][x as usize] = 'M';
                }

                // Is this the first column?
                else if x == 0 {
                    self.map[y as usize][x as usize] = 'M';
                }

                // Is this the last column?
                else if x == MAX_X {
                    self.map[y as usize][x as usize] = 'M';
                }
            }
        }
    }

    fn tile_has_player(&self, pos: Vec<i32>) -> bool {
        for p in &self.player_list {
            let player = p.player_pos.iter().zip(pos.iter()).filter(|(u, t)| u == t).count() == 2;
            if player { return true; }
        }

        false
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.tile_has_player(vec![y, x]) {
                    print!("O");
                } else {
                    print!("{}", &self.map[y as usize][x as usize]);
                }
                
                io::stdout().flush().unwrap();
            }

            println!("");
        }

        println!("");
    }
}

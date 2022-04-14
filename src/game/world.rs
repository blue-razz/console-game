use super::player::Player;
use colored::*;
use rand::distributions::Uniform;
use rand::prelude::*;
use std::io;
use std::io::Write;

struct Position(i32, i32);

pub struct World {
    world_name: String,
    pub height: i32,
    pub width: i32,
    pub map: Vec<Vec<char>>,
    player_list: Vec<Player>,
}

impl World {
    pub fn new(world_name: String, height: i32, width: i32) -> World {
        World {
            world_name,
            height,
            width,
            player_list: vec![],
            map: vec![vec!['.'; width as usize]; height as usize],
        }
    }

    pub fn spawn_player(&mut self, player_name: String) {
        self.player_list
            .push(Player::new(player_name, vec![16, 40]));
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
                } else {
                    // Generate a random numbeer between 0 and 100
                    let mut rng = rand::thread_rng();
                    let rand_num = Uniform::new(0, 100).sample(&mut rng);

                    // Generate more mountains as we get closer to the borders
                    if y < 6 || y > MAX_Y - 6 {
                        if rand_num > 50 {
                            self.map[y as usize][x as usize] = 'M';
                        }
                    } else if x < 6 || x > MAX_X - 6 {
                        if rand_num > 50 {
                            self.map[y as usize][x as usize] = 'M';
                        }
                    } else if y > 5 && y < 8 || y < MAX_Y - 8 && y > MAX_Y - 5 {
                        if rand_num > 90 {
                            self.map[y as usize][x as usize] = 'M';
                        }
                    } else if x > 5 && x < 12 || x > MAX_X - 12 && x < MAX_X - 5 {
                        if rand_num > 90 {
                            self.map[y as usize][x as usize] = 'M';
                        }
                    }
                    else {
                        if rand_num == 47 {
                            self.map[y as usize][x as usize] = 'M';
                        }
                    }
                    
                    // Generate rivers
                    if y > 5 && y < 8 && self.map[y as usize][x as usize] != 'M' {
                        if rand_num < 2 && self.map[y as usize][x as usize - 1] != 'R' {
                            self.map[y as usize][x as usize] = 'R';
                        }
                    } else {
                        if self.map[y as usize - 1][x as usize] == 'R' && self.map[y as usize][x as usize] != 'M' {
                            if rand_num > 25 {
                                self.map[y as usize][x as usize] = 'R';
                            } else {
                                self.map[y as usize][x as usize] = 'r';
                            }
                        }

                        if self.map[y as usize][x as usize - 1] == 'r' && self.map[y as usize][x as usize] != 'M' {
                            if rand_num > 45 {
                                self.map[y as usize][x as usize] = 'r';
                            } else {
                                self.map[y as usize][x as usize] = 'R';
                            }
                        }
                    }

                    if x > 6 && x < 7 || x < MAX_X - 6 && x > MAX_X - 7 {
                        if rand_num > 50 {
                            self.map[y as usize][x as usize] = 'r';
                        }
                    }

                    // Geneerate some trees
                    if y > 6 && y < MAX_Y - 6 && x > 6 && x < MAX_X - 6 {
                        if rand_num > 90 && self.map[y as usize][x as usize] != 'M' {
                            self.map[y as usize][x as usize] = 'T';
                        }
                    }
                }
            }
        }
    }

    fn tile_has_player(&self, pos: Vec<i32>) -> bool {
        for p in &self.player_list {
            let player = p
                .player_pos
                .iter()
                .zip(pos.iter())
                .filter(|(u, t)| u == t)
                .count()
                == 2;
            if player {
                return true;
            }
        }

        false
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.tile_has_player(vec![y, x]) {
                    print!("{}", "@".magenta().blink());
                } else {
                    if self.map[y as usize][x as usize] == 'M' {
                        print!("{}", "M".truecolor(255, 248, 220));
                    } else if self.map[y as usize][x as usize] == 'T' {
                        print!("{}", "T".bright_green().bold());
                    } else if self.map[y as usize][x as usize] == 'R' {
                        print!("{}", "|".blue().bold());
                    } else if self.map[y as usize][x as usize] == 'r' {
                        print!("{}", "-".blue().bold());
                    } else {
                        print!("{}", ".".white());
                    }
                }

                io::stdout().flush().unwrap();
            }

            println!("");
        }

        println!("");
    }
}

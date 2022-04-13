use std::io;
use std::io::Write;

struct Player {
    name: String,
    pos: Vec<i32>,
}

struct World {
    name: String,
    players: Vec<Player>,
    height: i32,
    width: i32,
}

impl World {
    fn spawn_player(&mut self, name: String) {
        self.players.push(Player {name, pos: Vec::from([&self.height/2, &self.width/2])});
    }
}

fn tile_has_player(position: Vec<i32>, world: &World) -> bool {
    for p in &world.players {
        println!("{}", p.name);
        if assert_eq!(position, p.pos) { 
            true;
        }
    }

    false
}

fn print_world(world: &World) {
    for i in 0..world.height+1 {
        print!("|");
        io::stdout().flush().unwrap();

        for j in 0..world.width {
            if i >= world.height {
                print!("_");
            } else if tile_has_player(Vec::from([world.height, world.width]), &world) {
                print!("o");
            } else {
                print!("*");
            }

            io::stdout().flush().unwrap();
        }

        
        println!();
    }


    println!();
}

fn main() {
    let mut world: World = World { name: String::from("overworld"), players: Vec::new(), height: 10, width: 30 };
    world.spawn_player(String::from("bob"));
    print_world(&world);
}

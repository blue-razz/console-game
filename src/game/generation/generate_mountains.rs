mod game;

use game::world::World;
use rand::prelude::*;
use rand::distributions::Uniform;



fn generate_borders(world: &mut World) {
    for y in 0..world.height {
        for x in 0..world.width {
            // Is this the first row?
            if y == 0 {
                world.map[y as usize][x as usize] = 'M';
            }
            // Is this the last row?
            else if y == world.max_y {
                world.map[y as usize][x as usize] = 'M';
            }
            // Is this the first column?
            else if x == 0 {
                world.map[y as usize][x as usize] = 'M';
            }
            // Is this the last column?
            else if x == world.max_x {
                world.map[y as usize][x as usize] = 'M';
            }
        }
    }
}

pub fn generate_mountains(world: &mut World) {
    generate_borders(world);


}

mod game;

use game::world::World;

fn main() {
    let mut world = World::new(String::from("overworld"), 50, 120);
    world.generate_world();
    world.spawn_player(String::from("bob"));
    world.print();
}

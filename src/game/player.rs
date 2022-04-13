use super::world::World;

pub struct Player {
    pub player_name: String,
    pub player_pos: Vec<i32>,
    pub player_health: f32,
}

impl Player {
    pub fn new(player_name: String, pos: Vec<i32>) -> Player {
        Player {
            player_name,
            player_pos: pos,
            player_health: 100.0,
        }
    }

    fn is_blocked(&self, world: &World, y: i32, x: i32) -> bool {
        // Is the player trying to move out of bounds?
        if x < 0 || y > world.height - 1 || y < 0 || x > world.width - 1 {
            return true;
        }

        // Is there an obstacle?
        match world.map[y as usize][x as usize] as char {
            'M' => true,
            _ => false,
        }
    }

    pub fn move_up(&mut self, tiles: i32, world: &World) {
        if self.is_blocked(world, self.player_pos[0] - tiles, self.player_pos[1]) {
            return;
        }

        self.player_pos[0] -= tiles;
    }

    pub fn move_down(&mut self, tiles: i32, world: &World) {
        if self.is_blocked(world, self.player_pos[0] + tiles, self.player_pos[1]) {
            return;
        }

        self.player_pos[0] += tiles;
    }

    pub fn move_left(&mut self, tiles: i32, world: &World) {
        if self.is_blocked(world, self.player_pos[0], self.player_pos[1] - tiles) {
            return;
        }

        self.player_pos[0] -= tiles;
    }

    pub fn move_right(&mut self, tiles: i32, world: &World) {
        if self.is_blocked(world, self.player_pos[0], self.player_pos[1] + tiles) {
            return;
        }

        self.player_pos[0] += tiles;
    }

    // Change the player's health
    pub fn set_health(&mut self, player_health: f32) {
        self.player_health = player_health;
    }
}

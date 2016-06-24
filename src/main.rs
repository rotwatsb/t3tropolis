extern crate kiss3d;
extern crate nalgebra;
extern crate glfw;
extern crate rand;
extern crate num;
extern crate rustc_serialize;

mod game;
mod multiplayer;

fn main() {
    game::play_tetris();
}




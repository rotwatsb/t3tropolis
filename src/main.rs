extern crate kiss3d;
extern crate nalgebra;
extern crate glfw;
extern crate rand;
extern crate num;
extern crate rustc_serialize;
extern crate bincode;

mod game;
mod multiplayer;
mod networkadapter;

fn main() {
    game::play_tetris();
}




extern crate kiss3d;
extern crate nalgebra;
extern crate glfw;

use nalgebra::{ Vector3, Rotation3, Isometry3 };

use kiss3d::window::Window;
use kiss3d::light::Light;

use glfw::{Action, WindowEvent, Key};

type Color = (f64, f64, f64);

const EColor: Color = (0.0, 0.0, 0.0);
const IColor: Color = (0.0, 1.0, 1.0);
const JColor: Color = (1.0, 1.0, 0.0);
const LColor: Color = (1.0, 0.647, 1.0);
const OColor: Color = (0.0, 0.0, 1.0);
const SColor: Color = (0.0, 1.0, 0.0);
const TColor: Color = (1.0, 0.0, 0.0);
const ZColor: Color = (0.5, 0.0, 0.5);

enum Cell {
    E, I, J, L, O, S, T, Z,
}

fn cell_color(p: &Cell) -> Color {
    match p {
        &Cell::E => EColor,
        &Cell::I => IColor,
        &Cell::J => JColor,
        &Cell::L => LColor,
        &Cell::O => OColor,
        &Cell::S => SColor,
        &Cell::T => TColor,
        &Cell::Z => ZColor,
    }
}

type Shape = [[[u8;4]; 4]; 4];

const IShape: Shape = [
    [[0, 1, 0, 0],
     [0, 1, 0, 0],
     [0, 1, 0, 0],
     [0, 1, 0, 0]],
    
    [[0, 0, 0, 0],
     [1, 1, 1, 1],
     [0, 0, 0, 0],
     [0, 0, 0, 0]],
    
    [[0, 1, 0, 0],
     [0, 1, 0, 0],
     [0, 1, 0, 0],
     [0, 1, 0, 0]],
    
    [[0, 0, 0, 0],
     [1, 1, 1, 1],
     [0, 0, 0, 0],
     [0, 0, 0, 0]]
];

const JShape: Shape = [
    [[0, 1, 0, 0],
     [0, 1, 0, 0],
     [1, 1, 0, 0],
     [0, 0, 0, 0]],
    
    [[1, 0, 0, 0],
     [1, 1, 1, 0],
     [0, 0, 0, 0],
     [0, 0, 0, 0]],
    
    [[0, 1, 1, 0],
     [0, 1, 0, 0],
     [0, 1, 0, 0],
     [0, 0, 0, 0]],
    
    [[0, 0, 0, 0],
     [1, 1, 1, 0],
     [0, 0, 1, 0],
     [0, 0, 0, 0]]
];

const LShape: Shape = [
    [[0, 1, 0, 0],
     [0, 1, 0, 0],
     [0, 1, 1, 0],
     [0, 0, 0, 0]],
    
    [[0, 0, 0, 0],
     [1, 1, 1, 0],
     [1, 0, 0, 0],
     [0, 0, 0, 0]],
    
    [[1, 1, 0, 0],
     [0, 1, 0, 0],
     [0, 1, 0, 0],
     [0, 0, 0, 0]],
    
    [[0, 0, 1, 0],
     [1, 1, 1, 0],
     [0, 0, 0, 0],
     [0, 0, 0, 0]]
];

const OShape: Shape = [
    [[1, 1, 0, 0],
     [1, 1, 0, 0],
     [0, 0, 0, 0],
     [0, 0, 0, 0]],
    
    [[1, 1, 0, 0],
     [1, 1, 0, 0],
     [0, 0, 0, 0],
     [0, 0, 0, 0]],
    
    [[1, 1, 0, 0],
     [1, 1, 0, 0],
     [0, 0, 0, 0],
     [0, 0, 0, 0]],
    
    [[1, 1, 0, 0],
     [1, 1, 0, 0],
     [0, 0, 0, 0],
     [0, 0, 0, 0]]
];

const SShape: Shape = [
    [[0, 1, 1, 0],
     [1, 1, 0, 0],
     [0, 0, 0, 0],
     [0, 0, 0, 0]],
    
    [[1, 0, 0, 0],
     [1, 1, 0, 0],
     [0, 1, 0, 0],
     [0, 0, 0, 0]],
    
    [[0, 1, 1, 0],
     [1, 1, 0, 0],
     [0, 0, 0, 0],
     [0, 0, 0, 0]],
    
    [[1, 0, 0, 0],
     [1, 1, 0, 0],
     [0, 1, 0, 0],
     [0, 0, 0, 0]]
];

const TShape: Shape = [
    [[0, 1, 0, 0],
     [1, 1, 1, 0],
     [0, 0, 0, 0],
     [0, 0, 0, 0]],
    
    [[0, 1, 0, 0],
     [0, 1, 1, 0],
     [0, 1, 0, 0],
     [0, 0, 0, 0]],

    [[0, 0, 0, 0],
     [1, 1, 1, 0],
     [0, 1, 0, 0],
     [0, 0, 0, 0]],
    
    [[0, 1, 0, 0],
     [1, 1, 0, 0],
     [0, 1, 0, 0],
     [0, 0, 0, 0]]
];

const ZShape: Shape = [
    [[1, 1, 0, 0],
     [0, 1, 1, 0],
     [0, 0, 0, 0],
     [0, 0, 0, 0]],
    
    [[0, 0, 1, 0],
     [0, 1, 1, 0],
     [0, 1, 0, 0],
     [0, 0, 0, 0]],
    
    [[1, 1, 0, 0],
     [0, 1, 1, 0],
     [0, 0, 0, 0],
     [0, 0, 0, 0]],
    
    [[0, 0, 1, 0],
     [0, 1, 1, 0],
     [0, 1, 0, 0],
     [0, 0, 0, 0]]
];


struct game {
    board: [[Cell; 10]; 22],
}

fn main() {
    let mut window = Window::new("Kiss3d: cube");
    let mut board = window.add_group();
    
    let mut c1 = board.add_cube(1.0, 1.0, 1.0);
    let mut c2 = board.add_cube(1.0, 1.0, 1.0);
    
    c1.prepend_to_local_translation(&Vector3::new(1.0, 0.0, 0.0));
    c2.prepend_to_local_translation(&Vector3::new(3.0, 0.0, 0.0));
    
    window.set_light(Light::StickToCamera);

    while window.render() {
        board.prepend_to_local_rotation(&Vector3::new(0.0f32, 0.014, 0.00));
        for mut event in window.events().iter() {
            match event.value {
                WindowEvent::Key(code, _, Action::Press, _) => {
                    match code {
                        Key::W =>
                            board.prepend_to_local_translation(
                                &Vector3::new(0.0, 1.0, 0.0)),
                        Key::S =>
                            board.prepend_to_local_translation(
                                &Vector3::new(0.0, -1.0, 0.0)),
                        _ => (),
                    }

                    event.inhibited = true // override the default keyboard handler
                },
                _ => (),
            }
        }
    }
}



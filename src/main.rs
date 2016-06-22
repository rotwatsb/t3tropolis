extern crate kiss3d;
extern crate nalgebra;
extern crate glfw;

use nalgebra::{ Vector3, Rotation3, Isometry3, Point3 };

use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;

use glfw::{Action, WindowEvent, Key};



type Color = (f32, f32, f32);

const EColor: Color = (0.0, 0.0, 0.0);
const IColor: Color = (0.0, 1.0, 1.0);
const JColor: Color = (1.0, 1.0, 0.0);
const LColor: Color = (1.0, 0.647, 1.0);
const OColor: Color = (0.0, 0.0, 1.0);
const SColor: Color = (0.0, 1.0, 0.0);
const TColor: Color = (1.0, 0.0, 0.0);
const ZColor: Color = (0.5, 0.0, 0.5);

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    E, I, J, L, O, S, T, Z,
}

fn cell_color(p: Cell) -> Color {
    match p {
        Cell::E => EColor,
        Cell::I => IColor,
        Cell::J => JColor,
        Cell::L => LColor,
        Cell::O => OColor,
        Cell::S => SColor,
        Cell::T => TColor,
        Cell::Z => ZColor,
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


struct Game {
    board: [[Cell; 10]; 22],
    board_grp: SceneNode,
    tetromino_grp: SceneNode,
}

impl Game {
    fn new(window: &mut Window) -> Game {
        let mut bg = window.add_group();
        let mut tg = bg.add_group();
        let mut g = Game {
            board: [[Cell::E; 10]; 22],
            board_grp: bg,
            tetromino_grp: tg,
        };
        g.board[0][0] = Cell::I;
        g.board[5][5] = Cell::I;
        g
    }

    fn draw(&mut self, window: &mut Window) {
        self.board_grp.unlink();
        self.tetromino_grp.unlink();
        self.board_grp = window.add_group();
        self.tetromino_grp = self.board_grp.add_group();
        self.board_grp.prepend_to_local_translation(&Vector3::new(0.0, 0.0, 30.0));
        
        for r in 0..22 {
            for c in 0..10 {
                if self.board[r][c] != Cell::E {
                    let mut cube = self.board_grp.add_cube(1.0, 1.0, 1.0);
                    cube.prepend_to_local_translation(
                        &Vector3::new(c as f32 - 4.5, r as f32 - 9.5, -4.5));
                    let color = cell_color(self.board[r][c]);
                    cube.set_color(color.0, color.1, color.2);
                }
            }
        }
    }
}

fn draw_grid(window: &mut Window, wt: &mut Isometry3<f32>) {
    for x in -5..6 {
        for z in -5..6 {
            let p1 = Point3::new(x as f32, -10.0, z as f32);
            let _p1 = *wt * p1;
            let p2 = Point3::new(x as f32, 10.0, z as f32);
            let _p2 = *wt * p2;
            let c = Point3::new(1.0 as f32, 1.0 as f32, 1.0 as f32);
            window.draw_line(&_p1, &_p2, &c);
        }
    }
    
    for y in -10..10 {
        for x in -5..6 {
            let p1 = Point3::new(x as f32, y as f32, -5.0 as f32);
            let _p1 = *wt * p1;
            let p2 = Point3::new(x as f32, y as f32, 5.0 as f32);
            let _p2 = *wt * p2;
            let c = Point3::new(1.0 as f32, 1.0 as f32, 1.0 as f32);
            window.draw_line(&_p1, &_p2, &c);
        }
        for z in -5..6 {
            let p1 = Point3::new(5.0 as f32, y as f32, z as f32);
            let _p1 = *wt * p1;
            let p2 = Point3::new(-5.0 as f32, y as f32, z as f32);
            let _p2 = *wt * p2;
            let c = Point3::new(1.0 as f32, 1.0 as f32, 1.0 as f32);
            window.draw_line(&_p1, &_p2, &c);
        }
     
    }
}

fn main() {
    let mut window = Window::new("T3tropolis!");

    let mut game = Game::new(&mut window);
    
    window.set_light(Light::StickToCamera);

    while window.render() {

        let mut wt = game.board_grp.data().world_transformation();
        draw_grid(&mut window, &mut wt);
        game.draw(&mut window);
        //board.prepend_to_local_rotation(&Vector3::new(0.0f32, 0.014, 0.00));
        for mut event in window.events().iter() {
/*            match event.value {
                WindowEvent::Key(code, _, Action::Press, _) => {
                    match code {
                        Key::W =>
                            tetromino.prepend_to_local_translation(
                                &Vector3::new(0.0, 1.0, 0.0)),
                        Key::S =>
                            tetromino.prepend_to_local_translation(
                                &Vector3::new(0.0, -1.0, 0.0)),
                        _ => (),
                    }

                    event.inhibited = true // override the default keyboard handler
                },
                _ => (),
            }*/
        }
    }
}



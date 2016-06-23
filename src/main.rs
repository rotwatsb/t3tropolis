extern crate kiss3d;
extern crate nalgebra;
extern crate glfw;
extern crate rand;

use std::time::{SystemTime};

use nalgebra::{ Vector3, Rotation3, Isometry3, Point3 };

use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;

use glfw::{Action, WindowEvent, Key};

use rand::{OsRng, Rng};


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

fn cell_of_shape(s: Shape) -> Cell {
    match s {
        IShape => Cell::I,
        JShape => Cell::J,
        LShape => Cell::L,
        OShape => Cell::O,
        SShape => Cell::S,
        TShape => Cell::T,
        ZShape => Cell::Z,
        _ => Cell::I,
    }
}

fn tetro_color(s: Shape) -> Color {
    match s {
        IShape => IColor,
        JShape => JColor,
        LShape => LColor,
        OShape => OColor,
        SShape => SColor,
        TShape => TColor,
        ZShape => ZColor,
        _ => IColor,
    }
}

type Shape = [[[u8; 4]; 4]; 4];

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
    rng: OsRng,
    tetromino: (Shape, usize),
    temp: [[u8; 4]; 4],
    next_tetromino: (Shape, usize),
    tetro_pos: (i8, i8),
}

impl Game {

    fn new(window: &mut Window) -> Game {
        let mut bg = window.add_group();
        let mut tg = bg.add_group();
        let mut g = Game {
            board: [[Cell::E; 10]; 22],
            board_grp: bg,
            tetromino_grp: tg,
            rng: OsRng::new().unwrap(),
            tetromino: (IShape, 0),
            temp: [[0; 4]; 4],
            next_tetromino: (IShape, 0),
            tetro_pos: (19, 5),
        };
        g
    }

    fn begin(&mut self) {
        self.select_next_shape();
        self.new_tetromino();
    }

    fn select_next_shape(&mut self) {
        self.tetromino =
            match (self.rng.next_f32() * 7.0) as u8 {
                0 => (IShape, 0),
                1 => (JShape, 0),
                2 => (LShape, 0),
                3 => (OShape, 0),
                4 => (SShape, 0),
                5 => (TShape, 0),
                6 => (ZShape, 0),
                _ => (IShape, 0),
            };
    }

    fn new_tetromino(&mut self) {
        self.tetromino = self.next_tetromino;
        self.select_next_shape();
    }

    fn rotate_tetromino(&mut self) {
        self.tetromino.1 = (self.tetromino.1 + 1) % 4;
        if self.collision(0,0) {
            self.tetromino.1 = (self.tetromino.1 - 1) % 4;
        }
    }

    fn move_down(&mut self) {
        self.tetro_pos.0 -= 1;

	if self.collision(0,0) {
	    self.tetro_pos.0 += 1;
	    self.tetro_to_board();
	    //self.clear_lines();
	    self.tetro_pos = (19, 5);
 	    self.new_tetromino();
	}
    }

    fn move_right(&mut self) {
        self.tetro_pos.1 -= 1;

	if self.collision(0,0) {
	    self.tetro_pos.1 += 1;
	}
    }

    fn move_left(&mut self) {
        self.tetro_pos.1 += 1;

	if self.collision(0,0) {
	    self.tetro_pos.1 -= 1;
	}
    }

    fn tetro_to_board(&mut self) {
        for i in 0..4 {
	    for j in 0..4 {
	        if self.tetromino.0[self.tetromino.1][i][j] != 0 {
		 //   if (board[i+sr][j+sc] > 0) {
		 //       lost = true;
		 //   }
		    self.board[(i as i8 + self.tetro_pos.0) as usize]
                        [(j as i8 + self.tetro_pos.1) as usize] =
                        cell_of_shape(self.tetromino.0);
	        }
            }
        }
    }

    fn collision(&mut self, dc: i8, dr: i8) -> bool {
        let nr = self.tetro_pos.0 + dr;
        let nc = self.tetro_pos.1 + dc;

        println!("nr: {} nc: {}", nr, nc);
        for i in 0..4 {
	    for j in 0..4 {
                println!("{} {}", i as i8 + nr, j as i8 + nc);
	        if self.tetromino.0[self.tetromino.1][i][j] != 0 {
		    if (i as i8 + nr) < 22 && (i as i8 + nr) >= 0 &&
                        (j as i8 + nc) < 10 && (j as i8 + nc) >= 0 {
		            if self.board[(i as i8 + nr) as usize]
                                [(j as i8 + nc) as usize] != Cell::E {
			            return true;
		                }
                        }
		    else {
		        return true;
		    }
                }
            }
        }
        false
    }

    fn draw(&mut self, window: &mut Window) {
        self.board_grp.unlink();
        self.tetromino_grp.unlink();
        self.board_grp = window.add_group();
        self.tetromino_grp = self.board_grp.add_group();
        self.board_grp.prepend_to_local_translation(&Vector3::new(0.0, 0.0, 30.0));

        self.draw_board();
        self.draw_tetromino();
    }

    fn draw_board(&mut self) {
        for r in 0..22 {
            for c in 0..10 {
                if self.board[r][c] != Cell::E {
                    let mut cube = self.board_grp.add_cube(0.8, 0.8, 0.8);
                    cube.prepend_to_local_translation(
                        &Vector3::new(c as f32 - 4.5, r as f32 - 9.5, -4.5));
                    let color = cell_color(self.board[r][c]);
                    cube.set_color(color.0, color.1, color.2);
                }
            }
        }
    }

    fn draw_tetromino(&mut self) {
        for r in 0..4 {
            for c in 0..4 {
                if self.tetromino.0[self.tetromino.1][r][c] != 0 {
                    let mut cube = self.tetromino_grp.add_cube(0.8, 0.8, 0.8);
                    cube.prepend_to_local_translation(
                        &Vector3::new((self.tetro_pos.1 + c as i8) as f32 - 4.5,
                                      (self.tetro_pos.0 + r as i8) as f32 - 9.5,
                                      -4.5));
                    let color = tetro_color(self.tetromino.0);
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
    game.begin();
    
    window.set_light(Light::StickToCamera);

    let mut t = SystemTime::now();

    while window.render() {

        let mut wt = game.board_grp.data().world_transformation();

        game.draw(&mut window);
        draw_grid(&mut window, &mut wt);
        
        for mut event in window.events().iter() {
            match event.value {
                WindowEvent::Key(code, _, Action::Press, _) => {
                    match code {
                        Key::W | Key::Up =>
                            game.rotate_tetromino(),
                        Key::S | Key::Down =>
                            game.move_down(),
                        Key::A | Key::Left =>
                            game.move_left(),
                        Key::D | Key::Right =>
                            game.move_right(),
                        _ => (),
                    }

                    event.inhibited = true // override the default keyboard handler
                },
                _ => (),
            }
        }
         
    }
}



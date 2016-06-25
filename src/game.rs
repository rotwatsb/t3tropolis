use std::time::{ Duration, SystemTime };

use nalgebra::{ Vector3, Rotation, Isometry3, Point3 };

use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::camera::{ Camera, FirstPerson };

use glfw::{Action, WindowEvent, Key};

use rand::{OsRng, Rng};

use num::traits::One;

use multiplayer::{Mp, PlayerState};

pub const ROWS: usize = 22;
pub const COLS: usize = 10;
const CUBE_SIZE: f32 = 0.8;

type Color = (f32, f32, f32);

const EColor: Color = (0.0, 0.0, 0.0);
const IColor: Color = (0.0, 1.0, 1.0);
const JColor: Color = (1.0, 1.0, 0.0);
const LColor: Color = (1.0, 0.647, 1.0);
const OColor: Color = (0.0, 0.0, 1.0);
const SColor: Color = (0.0, 1.0, 0.0);
const TColor: Color = (1.0, 0.0, 0.0);
const ZColor: Color = (0.5, 0.0, 0.5);

#[derive(Copy, Clone, PartialEq, RustcDecodable, RustcEncodable, Debug)]
pub enum Cell {
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

pub type Shape = [[[u8; 4]; 4]; 4];

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

pub struct Game {
    pub board: [[Cell; COLS]; ROWS],
    orientation: Isometry3<f32>,
    board_grp: SceneNode,
    tetromino_grp: SceneNode,
    rng: OsRng,
    pub tetromino: (Shape, usize),
    pub next_tetromino: (Shape, usize),
    pub tetro_pos: (i8, i8),
    mp: Mp,
}

impl Game {

    fn new(window: &mut Window) -> Game {
        let mut bg = window.add_group();
        let mut tg = bg.add_group();
        let mut g = Game {
            board: [[Cell::E; COLS]; ROWS],
            orientation: Isometry3::one(),
            board_grp: bg,
            tetromino_grp: tg,
            rng: OsRng::new().unwrap(),
            tetromino: (IShape, 0),
            next_tetromino: (IShape, 0),
            tetro_pos: (ROWS as i8 - 3,  COLS as i8 / 2 - 1),
            mp: Mp::new(),
        };
        g.mp.open_peers();
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
            self.tetromino.1 = (self.tetromino.1 + 3) % 4;
        }
    }

    fn update_peers(&mut self) {
        let my_ps = PlayerState::new(self);
        self.mp.issue_update(&my_ps);
        self.mp.get_updates();
    }

    fn move_down(&mut self) {
        self.tetro_pos.0 -= 1;

	if self.collision(0,0) {
	    self.tetro_pos.0 += 1;
	    self.tetro_to_board();
	    self.clear_lines();
	    self.tetro_pos = (ROWS as i8 - 3, COLS as i8 / 2 - 1);
 	    self.new_tetromino();
	}
        self.update_peers();
    }

    fn drop(&mut self) {
        while !self.collision(-1, 0) {//&& self.tetro_pos.0 > 0 {
            self.tetro_pos.0 -= 1;
        }
    }

    fn clear_lines(&mut self) {
        let mut clear_line = true;
        for mut i in 0..ROWS {
	    clear_line = true;
	    for j in 0..COLS {
	        if self.board[ROWS - i - 1][j] == Cell::E {
		    clear_line = false;
		    break;
	        }
            }
	    if clear_line {
	    	self.delete_line(ROWS - i - 1);
	    }
        }
    }

    fn delete_line(&mut self, line: usize) {
        for i in line..(ROWS - 1) {
	    for j in 0..COLS {
	        self.board[i][j] = self.board[i + 1][j];
	    }
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

    fn collision(&mut self, dr: i8, dc: i8) -> bool {
        let nr = self.tetro_pos.0 + dr;
        let nc = self.tetro_pos.1 + dc;

        for i in 0..4 {
	    for j in 0..4 {
 	        if self.tetromino.0[self.tetromino.1][i][j] != 0 {
		    if (i as i8 + nr) < ROWS as i8 && (i as i8 + nr) >= 0 &&
                        (j as i8 + nc) < COLS as i8 && (j as i8 + nc) >= 0 {
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
        self.board_grp.prepend_to_local_transformation(&self.orientation);

        self.draw_boards();
        self.draw_tetrominos();
    }

    fn draw_boards(&mut self) {
        fn draw_board(board: &[[Cell; COLS]; ROWS], id: usize, board_grp: &mut SceneNode) {
            for r in 0..ROWS {
                for c in 0..COLS {
                    if board[r][c] != Cell::E {
                        let mut cube =
                            board_grp.add_cube(CUBE_SIZE, CUBE_SIZE, CUBE_SIZE);
                        cube.prepend_to_local_translation(
                            &Vector3::new(c as f32 - (COLS as f32 / 2.0 - 0.5),
                                          r as f32 - (ROWS as f32 / 2.0 - 0.5),
                                          -(COLS as f32 / 2.0 - 0.5) + id as f32));
                        let color = cell_color(board[r][c]);
                        cube.set_color(color.0, color.1, color.2);
                    }
                }
            }
        }
        draw_board(&self.board, 0, &mut self.board_grp);
        for ps in self.mp.peer_states.iter_mut() {
            println!("EH!!!");
            draw_board(&ps.board, ps.id, &mut self.board_grp);
        }
    }

    fn draw_tetrominos(&mut self) {
        fn draw_tetromino(tetromino: &(Shape, usize), tetro_pos: &(i8, i8),
                          id: usize, tetromino_grp: &mut SceneNode) {
            for r in 0..4 {
                for c in 0..4 {
                    if tetromino.0[tetromino.1][r][c] != 0 {
                        let mut cube =
                            tetromino_grp.add_cube(CUBE_SIZE, CUBE_SIZE, CUBE_SIZE);
                        cube.prepend_to_local_translation(
                            &Vector3::new((tetro_pos.1 + c as i8) as f32 -
                                          (COLS as f32 / 2.0 - 0.5),
                                          (tetro_pos.0 + r as i8) as f32 -
                                          (ROWS as f32 / 2.0 - 0.5),
                                          -(COLS as f32 / 2.0 - 0.5)));
                        let color = tetro_color(tetromino.0);
                        cube.set_color(color.0, color.1, color.2);
                    }
                }
            }
        }
        draw_tetromino(&self.tetromino, &self.tetro_pos, 0, &mut self.tetromino_grp);
    }

}

fn draw_grid(window: &mut Window, wt: &mut Isometry3<f32>) {
    for x in -(COLS as isize / 2)..(COLS as isize / 2 + 1) {
        for z in -(COLS as isize / 2)..(COLS as isize / 2 + 1) {
            let p1 = Point3::new(x as f32, -(ROWS as f32 / 2.0), z as f32);
            let _p1 = *wt * p1;
            let p2 = Point3::new(x as f32, (ROWS as f32 / 2.0), z as f32);
            let _p2 = *wt * p2;
            let c = Point3::new(0.5 as f32, 0.5 as f32, 0.5 as f32);
            window.draw_line(&_p1, &_p2, &c);
        }
    }
    
    for y in -(ROWS as isize / 2)..(ROWS as isize / 2) {
        for x in -(COLS as isize / 2)..(COLS as isize / 2 + 1) {
            let p1 = Point3::new(x as f32, y as f32, -(COLS as f32 / 2.0));
            let _p1 = *wt * p1;
            let p2 = Point3::new(x as f32, y as f32, COLS as f32 / 2.0);
            let _p2 = *wt * p2;
            let c = Point3::new(0.5 as f32, 0.5 as f32, 0.5 as f32);
            window.draw_line(&_p1, &_p2, &c);
        }
        for z in -(COLS as isize / 2)..(COLS as isize / 2 + 1) {
            let p1 = Point3::new(COLS as f32 / 2.0, y as f32, z as f32);
            let _p1 = *wt * p1;
            let p2 = Point3::new(-(COLS as f32 / 2.0), y as f32, z as f32);
            let _p2 = *wt * p2;
            let c = Point3::new(0.5 as f32, 0.5 as f32, 0.5 as f32);
            window.draw_line(&_p1, &_p2, &c);
        }
        
    }
}

pub fn play_tetris() {
    let mut window = Window::new("T3tropolis!");
    window.set_light(Light::StickToCamera);

    let mut game = Game::new(&mut window);
    game.begin();
    
    

    //let mut mycam = FirstPerson::new(Point3::new(0.0, 0.0, -30.0),
    //                                 Point3::new(0.0, 0.0, 0.0));
    

    let mut t1 = SystemTime::now();

    let mut mouse_pos: (f64, f64) = (0.0, 0.0);
    let mut mouse_press_pos: (f64, f64) = (0.0, 0.0);
    let mut rotate_board = false;

    //while window.render_with_camera(&mut mycam) {
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
                        Key::Space =>
                            game.drop(),
                        _ => (),
                    }

                    event.inhibited = true // override the default keyboard handler
                },
                WindowEvent::MouseButton(button, Action::Press, mods) => {
                    rotate_board = true;
                    mouse_press_pos = mouse_pos;
                    event.inhibited = true // override the default mouse handler
                },
                WindowEvent::MouseButton(button, Action::Release, mods) => {
                    rotate_board = false;
                    event.inhibited = true // override the default mouse handler
                },
                WindowEvent::CursorPos(x, y) => {
                    mouse_pos = (x, y);
                    if rotate_board {
                        game.orientation.prepend_rotation_mut(
                            &Vector3::new(//((mouse_pos.0 - mouse_press_pos.0) /
                                // 1000.0) as f32,
                                0.0,
                                ((mouse_pos.1 - mouse_press_pos.1) /
                                 1000.0) as f32,
                                //0.0,
                                0.0));
                    }

                    event.inhibited = true // override the default mouse handler
                },
                _ => (),
            }
        }
        if let Ok(d) = SystemTime::now().duration_since(t1) {
            if d.as_secs() > 0.5 as u64 {
                game.move_down();
                t1 = SystemTime::now();
            }
        }

    }
}

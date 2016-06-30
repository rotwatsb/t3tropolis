use std::cmp;
use std::f32;

use rand::{OsRng, Rng};

use kiss3d::window::Window;
use kiss3d::scene::SceneNode;

use nalgebra::{ Vector3, Isometry3, Point3 };

use num::traits::One;

const CUBE_SIZE: f32 = 0.8;
pub const ROWS: usize = 22;
pub const COLS: usize = 10;

pub type Color = (f32, f32, f32);

const ECOLOR: Color = (0.0, 0.0, 0.0);
const ICOLOR: Color = (0.0, 1.0, 1.0);
const JCOLOR: Color = (1.0, 1.0, 0.0);
const LCOLOR: Color = (1.0, 0.647, 1.0);
const OCOLOR: Color = (0.0, 0.0, 1.0);
const SCOLOR: Color = (0.0, 1.0, 0.0);
const TCOLOR: Color = (1.0, 0.0, 0.0);
const ZCOLOR: Color = (0.5, 0.0, 0.5);

#[derive(Copy, Clone, PartialEq, RustcDecodable, RustcEncodable, Debug)]
pub enum Cell {
    E, I, J, L, O, S, T, Z,
}

fn tetro_color(s: Shape) -> Color {
    match s {
        ISHAPE => ICOLOR,
        JSHAPE => JCOLOR,
        LSHAPE => LCOLOR,
        OSHAPE => OCOLOR,
        SSHAPE => SCOLOR,
        TSHAPE => TCOLOR,
        ZSHAPE => ZCOLOR,
        _ => ICOLOR,
    }
}

fn cell_color(p: Cell) -> Color {
    match p {
        Cell::E => ECOLOR,
        Cell::I => ICOLOR,
        Cell::J => JCOLOR,
        Cell::L => LCOLOR,
        Cell::O => OCOLOR,
        Cell::S => SCOLOR,
        Cell::T => TCOLOR,
        Cell::Z => ZCOLOR,
    }
}

fn cell_of_shape(s: Shape) -> Cell {
    match s {
        ISHAPE => Cell::I,
        JSHAPE => Cell::J,
        LSHAPE => Cell::L,
        OSHAPE => Cell::O,
        SSHAPE => Cell::S,
        TSHAPE => Cell::T,
        ZSHAPE => Cell::Z,
        _ => Cell::I,
    }
}


pub type Shape = [[[u8; 4]; 4]; 4];

pub const ISHAPE: Shape = [
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

const JSHAPE: Shape = [
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

const LSHAPE: Shape = [
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

const OSHAPE: Shape = [
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

const SSHAPE: Shape = [
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

const TSHAPE: Shape = [
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

const ZSHAPE: Shape = [
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

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct PlayerState {
    pub board: [[Cell; COLS]; ROWS],
    pub tetromino: (Shape, usize),
    pub next_tetromino: (Shape, usize, TradeState),
    pub tetro_pos: (i8, i8),
    pub id: usize,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub enum TradeState {
    NoTrade,
    Pending(usize),
    Confirm(usize),
}

impl PlayerState {
    pub fn new(id: usize) -> PlayerState {
        PlayerState {
            board: [[Cell::E; COLS]; ROWS],
            tetromino: (ISHAPE, 0),
            next_tetromino: (ISHAPE, 0, TradeState::NoTrade),
            tetro_pos: (ROWS as i8 - 3, COLS as i8 / 2 - 1),
            id: id,
        }
    }

    pub fn begin(&mut self) {
        self.select_next_shape();
        self.new_tetromino();
    }

    fn select_next_shape(&mut self) {
        let mut rng = OsRng::new().unwrap();
        self.next_tetromino =
            match (rng.next_f32() * 7.0) as u8 {
                0 => (ISHAPE, 0, TradeState::NoTrade),
                1 => (JSHAPE, 0, TradeState::NoTrade),
                2 => (LSHAPE, 0, TradeState::NoTrade),
                3 => (OSHAPE, 0, TradeState::NoTrade),
                4 => (SSHAPE, 0, TradeState::NoTrade),
                5 => (TSHAPE, 0, TradeState::NoTrade),
                6 => (ZSHAPE, 0, TradeState::NoTrade),
                _ => (ISHAPE, 0, TradeState::NoTrade),
            };
    }

    fn new_tetromino(&mut self) {
        self.tetromino.0 = self.next_tetromino.0;
        self.tetromino.1 = self.next_tetromino.1;
        self.select_next_shape();
    }

    pub fn toggle_swap(&mut self, d: isize, n: isize) {
        let target: usize = ((self.id as isize + d + n) % n) as usize;
        match self.next_tetromino.2 {
            TradeState::NoTrade =>
                self.next_tetromino.2 = TradeState::Pending(target),
            TradeState::Pending(id) => {
                if id == target {
                    self.next_tetromino.2 = TradeState::NoTrade;
                }
                else {
                    self.next_tetromino.2 = TradeState::Pending(target);
                }
            },
            TradeState::Confirm(_) => (),            
        }
    }

    pub fn rotate_tetromino(&mut self) {
        self.tetromino.1 = (self.tetromino.1 + 1) % 4;
        if self.collision(0,0) {
            self.tetromino.1 = (self.tetromino.1 + 3) % 4;
        }
    }

    pub fn move_down(&mut self) {
        self.tetro_pos.0 -= 1;

	if self.collision(0,0) {
	    self.tetro_pos.0 += 1;
	    self.tetro_to_board();
	    self.clear_lines();
	    self.tetro_pos = (ROWS as i8 - 3, COLS as i8 / 2 - 1);
 	    self.new_tetromino();
	}
    }

    pub fn drop(&mut self) {
        while !self.collision(-1, 0) {//&& self.tetro_pos.0 > 0 {
            self.tetro_pos.0 -= 1;
        }
    }

    fn clear_lines(&mut self) {
        let mut clear_line = true;
        for i in 0..ROWS {
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

    pub fn move_right(&mut self) {
        self.tetro_pos.1 -= 1;

	if self.collision(0,0) {
	    self.tetro_pos.1 += 1;
	}
    }

    pub fn move_left(&mut self) {
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
}

pub struct Graphics {
    pub orientation: Isometry3<f32>,
    pub board_grp: SceneNode,
    pub tetromino_grp: SceneNode,
}

impl Graphics {
    pub fn new(window: &mut Window) -> Graphics {
        let mut bg = window.add_group();
        let tg = bg.add_group();
        Graphics {
            orientation: Isometry3::one(),
            board_grp: bg,
            tetromino_grp: tg,
        }
    }

    pub fn draw(&mut self, window: &mut Window,
            player_states: &Vec<PlayerState>, my_id: usize) {
        self.board_grp.unlink();
        self.tetromino_grp.unlink();
        self.board_grp = window.add_group();
        self.tetromino_grp = self.board_grp.add_group();
        self.board_grp.prepend_to_local_translation(&Vector3::new(0.0, 0.0, 31.0));
        self.board_grp.prepend_to_local_transformation(&self.orientation);

        self.draw_boards(player_states, my_id);
        self.draw_tetrominos(player_states, my_id);
        self.draw_nexts(player_states, my_id as isize);
    }

    fn draw_nexts(&mut self, player_states: &Vec<PlayerState>, my_id: isize) {
        let trade_id = match player_states[my_id as usize].next_tetromino.2 {
            TradeState::NoTrade => my_id as usize,
            TradeState::Pending(o_id) => o_id,
            TradeState::Confirm(o_id) => o_id,
        };
        
        let num_players = player_states.len() as isize;
        let span = cmp::min(3, num_players);
        for i in -(span / 2)..f32::ceil(span as f32 / 2.0) as isize {
            let id = (my_id + i + num_players) % num_players;
            let tetromino = player_states[id as usize].next_tetromino.clone();


            for r in 0..4 {
                for c in 0..4 {
                    if tetromino.0[tetromino.1][r][c] != 0 {
                        let mut cube =
                            self.tetromino_grp.add_cube(CUBE_SIZE,
                                                        CUBE_SIZE, CUBE_SIZE);
                        let (x, y, z) =
                            ((c as isize - 4 as isize - (COLS / 2) as isize) as f32,
                             r as f32 + i as f32 * 5.0,
                             -(COLS as f32 / 2.0 - 0.5) +
                             ((id as i8 - my_id as i8 + num_players as i8)
                              % num_players as i8) as f32);
                        cube.prepend_to_local_translation(&Vector3::new(x, y, z));
                        let color = tetro_color(tetromino.0);
                        cube.set_color(color.0, color.1, color.2);
                        if trade_id as isize == id {
                            let mut cube =
                                self.tetromino_grp.add_cube(CUBE_SIZE,
                                                            CUBE_SIZE, CUBE_SIZE);
                            cube.prepend_to_local_translation(&Vector3::new(x, y, z + 1.0));
                            cube.set_color(1.0, 1.0, 1.0);
                        }
                    }
                }
            }
        }
    }

    fn draw_boards(&mut self, player_states: &Vec<PlayerState>, my_id: usize) {
        fn draw_board(board: &[[Cell; COLS]; ROWS], id: usize,
                      board_grp: &mut SceneNode, my_id: usize, num_players: usize) {
            for r in 0..ROWS {
                for c in 0..COLS {
                    if board[r][c] != Cell::E {
                        let mut cube =
                            board_grp.add_cube(CUBE_SIZE, CUBE_SIZE, CUBE_SIZE);
                        cube.prepend_to_local_translation(
                            &Vector3::new(c as f32 - (COLS as f32 / 2.0 - 0.5),
                                          r as f32 - (ROWS as f32 / 2.0 - 0.5),
                                          -(COLS as f32 / 2.0 - 0.5) +
                                          ((id as i8 - my_id as i8
                                            + num_players as i8)
                                           % num_players as i8) as f32));
                        let color = cell_color(board[r][c]);
                        cube.set_color(color.0, color.1, color.2);
                    }
                }
            }
        }
        let l = player_states.len();
        for ps in player_states {
            draw_board(&ps.board, ps.id, &mut self.board_grp, my_id, l);
        }
    }

    fn draw_tetrominos(&mut self, player_states: &Vec<PlayerState>, my_id: usize) {
        fn draw_tetromino(tetromino: &(Shape, usize), tetro_pos: &(i8, i8),
                          id: usize, tetromino_grp: &mut SceneNode, my_id: usize,
                          num_players: usize) {
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
                                          -(COLS as f32 / 2.0 - 0.5) +
                                          ((id as i8 - my_id as i8
                                            + num_players as i8)
                                           % num_players as i8) as f32));
                        let color = tetro_color(tetromino.0);
                        cube.set_color(color.0, color.1, color.2);
                    }
                }
            }
        }
        let l = player_states.len();
        for ps in player_states {
            draw_tetromino(&ps.tetromino, &ps.tetro_pos,
                           ps.id, &mut self.board_grp, my_id, l);
        }
    }

    pub fn draw_grid(&self, window: &mut Window) {
        let wt = self.board_grp.data().world_transformation();
        for x in -(COLS as isize / 2)..(COLS as isize / 2 + 1) {
            for z in -(COLS as isize / 2)..(COLS as isize / 2 + 1) {
                let p1 = Point3::new(x as f32, -(ROWS as f32 / 2.0), z as f32);
                let _p1 = wt * p1;
                let p2 = Point3::new(x as f32, (ROWS as f32 / 2.0), z as f32);
                let _p2 = wt * p2;
                let c = Point3::new(0.5 as f32, 0.5 as f32, 0.5 as f32);
                window.draw_line(&_p1, &_p2, &c);
            }
        }
        
        for y in -(ROWS as isize / 2)..(ROWS as isize / 2) {
            for x in -(COLS as isize / 2)..(COLS as isize / 2 + 1) {
                let p1 = Point3::new(x as f32, y as f32, -(COLS as f32 / 2.0));
                let _p1 = wt * p1;
                let p2 = Point3::new(x as f32, y as f32, COLS as f32 / 2.0);
                let _p2 = wt * p2;
                let c = Point3::new(0.5 as f32, 0.5 as f32, 0.5 as f32);
                window.draw_line(&_p1, &_p2, &c);
            }
            for z in -(COLS as isize / 2)..(COLS as isize / 2 + 1) {
                let p1 = Point3::new(COLS as f32 / 2.0, y as f32, z as f32);
                let _p1 = wt * p1;
                let p2 = Point3::new(-(COLS as f32 / 2.0), y as f32, z as f32);
                let _p2 = wt * p2;
                let c = Point3::new(0.5 as f32, 0.5 as f32, 0.5 as f32);
                window.draw_line(&_p1, &_p2, &c);
            }
            
        }
    }
}



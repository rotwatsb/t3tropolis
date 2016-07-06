use std::cmp;
use std::f32;
use std::rc::Rc;
use std::cell::{RefCell};
use std::path::Path;

use kiss3d::window::Window;
use kiss3d::scene::SceneNode;
use kiss3d::resource::material::Matrixerial;
use kiss3d::text::Font;

use nalgebra::{Vector3, Isometry3, Point2, Point3, Rotation};

use num::traits::One;

use playerstate::{PlayerState, Shape, ISHAPE, JSHAPE, LSHAPE, OSHAPE,
                  SSHAPE, TSHAPE, ZSHAPE, Cell, ROWS, COLS, TradeState};

use other_material::MyObjectMatrixerial;

const CUBE_SIZE: f32 = 0.8;

pub type Color = (f32, f32, f32);

const ECOLOR: Color = (0.0, 0.0, 0.0);
const ICOLOR: Color = (0.0, 1.0, 1.0);
const JCOLOR: Color = (1.0, 1.0, 0.0);
const LCOLOR: Color = (1.0, 0.647, 1.0);
const OCOLOR: Color = (0.0, 0.0, 1.0);
const SCOLOR: Color = (0.0, 1.0, 0.0);
const TCOLOR: Color = (1.0, 0.0, 0.0);
const ZCOLOR: Color = (0.5, 0.0, 0.5);

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

pub struct Draw {
    pub orientation: Isometry3<f32>,
    pub board_grp: SceneNode,
    pub tetromino_grp: SceneNode,
    pub translucent_mat: Rc<RefCell<Box<Matrixerial>>>,
    pub opaque_mat: Rc<RefCell<Box<Matrixerial>>>,
    anim_frames: u32,
    anim_frame_count: u32,
    anim_rot_vec: Vector3<f32>,
}

impl Draw {
    pub fn new(window: &mut Window) -> Draw {
        let mut bg = window.add_group();
        let tg = bg.add_group();
        Draw {
            orientation: Isometry3::one(),
            board_grp: bg,
            tetromino_grp: tg,
            translucent_mat: Rc::new(RefCell::new(Box::new(
                MyObjectMatrixerial::new(true)))),
            opaque_mat: Rc::new(RefCell::new(Box::new(
                MyObjectMatrixerial::new(false)))),
            anim_frames: 0,
            anim_frame_count: 0,
            anim_rot_vec: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn animating(&self) -> bool {
        self.anim_frame_count > 0
    }

    pub fn draw(&mut self, window: &mut Window,
                player_states: &Vec<PlayerState>, my_id: usize, score: u32) {
        self.board_grp.unlink();
        self.tetromino_grp.unlink();
        self.board_grp = window.add_group();
        self.tetromino_grp = self.board_grp.add_group();
        self.board_grp.prepend_to_local_translation(&Vector3::new(0.0, 0.0, 31.0));
        self.board_grp.prepend_to_local_transformation(&self.orientation);

        if self.anim_frame_count > 0 {
            self.anim_frame_count -= 1;
            self.orientation.prepend_rotation_mut(&self.anim_rot_vec);
        }

        self.draw_grid(window);
        self.draw_boards(player_states, my_id);
        self.draw_tetrominos(player_states, my_id);
        self.draw_nexts(player_states, my_id as isize);
        self.draw_score(window, score);
    }

    pub fn anim_rot(&mut self, rot_angle: f32, frames: u32) {
        self.anim_rot_vec.y = rot_angle / (frames as f32);
        self.anim_frames = frames;
        self.anim_frame_count = self.anim_frames;
    }

    fn draw_score(&self, window: &mut Window, score: u32) {
        let font = Font::new(&Path::new("./src/FreeSans.ttf"), 60);
        window.draw_text(&score.to_string(), &Point2::new(0.0, 80.0),
                         &font, &Point3::new(0.0, 0.0, 1.0));
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
            let ref tetromino = player_states[id as usize].next_tetromino;

            let z = ((id as i8 - my_id as i8 + num_players as i8)
                     % num_players as i8) as f32 - (COLS as f32 / 2.0 - 0.5);

            for r in 0..4 {
                for c in 0..4 {
                    if tetromino.0[tetromino.1][r][c] != 0 {
                        let mut cube =
                            self.tetromino_grp.add_cube(CUBE_SIZE,
                                                        CUBE_SIZE,
                                                        CUBE_SIZE);
                        let (mut x, y) =
                            ((c as isize - 4 as isize - (COLS / 2) as isize) as f32,
                             r as f32 + i as f32 * 5.0);
                        
                        if let TradeState::Pending(peer_target) = tetromino.2 {
                            if peer_target == my_id as usize && id != my_id {
                                x -= 5.0;
                            }
                        }
                       
                        cube.prepend_to_local_translation(&Vector3::new(x, y, z));
                        let color = tetro_color(tetromino.0);
                        cube.set_color(color.0, color.1, color.2);
                        cube.set_material(self.opaque_mat.clone());

                        if trade_id as isize == id {
                            let mut cube =
                                self.tetromino_grp.add_cube(CUBE_SIZE,
                                                            CUBE_SIZE,
                                                            CUBE_SIZE);
                            cube.prepend_to_local_translation(
                                &Vector3::new(x, y, z + 1.0));
                            cube.set_color(1.0, 1.0, 1.0);
                            cube.set_material(self.opaque_mat.clone());
                        }
                    }
                }
            }
        }
    }

    fn draw_boards(&mut self, player_states: &Vec<PlayerState>, my_id: usize) {

        let num_players = player_states.len();

        for ps in player_states {
            for r in 0..ROWS {
                for c in 0..COLS {
                    if ps.board[r][c] != Cell::E {
                        let mut cube =
                            self.board_grp.add_cube(CUBE_SIZE, CUBE_SIZE, CUBE_SIZE);
                        cube.prepend_to_local_translation(
                            &Vector3::new(c as f32 - (COLS as f32 / 2.0 - 0.5),
                                          r as f32 - (ROWS as f32 / 2.0 - 0.5),
                                          -(COLS as f32 / 2.0 - 0.5) +
                                          ((ps.id as i8 - my_id as i8
                                            + num_players as i8)
                                           % num_players as i8) as f32));
                        let color = cell_color(ps.board[r][c]);
                        cube.set_color(color.0, color.1, color.2);
                        if ps.id == my_id {
                            cube.set_material(self.opaque_mat.clone());
                        }
                        else {
                            cube.set_material(self.translucent_mat.clone());
                        }
                    }
                }
            }
        }
    }

    fn draw_tetrominos(&mut self, player_states: &Vec<PlayerState>, my_id: usize) {

        let num_players = player_states.len();
        for ps in player_states {
            for r in 0..4 {
                for c in 0..4 {
                    let ref tetromino = ps.tetromino;
                    if tetromino.0[tetromino.1][r][c] != 0 {
                        let mut cube =
                            self.tetromino_grp.add_cube(
                                CUBE_SIZE, CUBE_SIZE, CUBE_SIZE);
                        cube.prepend_to_local_translation(
                            &Vector3::new((ps.tetro_pos.1 + c as i8) as f32 -
                                          (COLS as f32 / 2.0 - 0.5),
                                          (ps.tetro_pos.0 + r as i8) as f32 -
                                          (ROWS as f32 / 2.0 - 0.5),
                                          -(COLS as f32 / 2.0 - 0.5) +
                                          ((ps.id as i8 - my_id as i8
                                            + num_players as i8)
                                           % num_players as i8) as f32));
                        let color = tetro_color(tetromino.0);
                        cube.set_color(color.0, color.1, color.2);
                        if ps.id == my_id {
                            cube.set_material(self.opaque_mat.clone());
                        }
                        else {
                            cube.set_material(self.translucent_mat.clone());
                        }
                    }
                }
            }
        }
    }

    fn draw_grid(&self, window: &mut Window) {
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

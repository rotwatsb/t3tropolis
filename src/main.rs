extern crate kiss3d;
extern crate nalgebra;
extern crate glfw;
extern crate rand;
extern crate num;
extern crate rustc_serialize;
extern crate bincode;
extern crate gl;

mod playerstate;
mod draw;
mod multiplayer;
mod networkadapter;
mod other_material;

use playerstate::{PlayerState, TradeState, BoardState, Shape};
use draw::Draw;
use multiplayer::Mp;
use networkadapter::*;

use kiss3d::window::Window;
use kiss3d::light::Light;

use nalgebra::{Vector3, Rotation};

use glfw::{Action, WindowEvent, Key};

use std::time::{SystemTime};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {

    let mut mp: Mp = Mp::new();

    let peer_states: Arc<Mutex<Vec<PlayerState>>> =
        Arc::new(Mutex::new(vec![PlayerState::new(0); mp.id + 1]));
    
    let mut preserved_states: Vec<PlayerState> = Vec::new();
    
    let mut my_state: PlayerState = PlayerState::new(mp.id);

    let mut window = Window::new("T3tropolis");
    window.set_light(Light::StickToCamera);

    let mut drawer = Draw::new(&mut window);
    my_state.begin();

    let mut stream_read = mp.connection.try_clone().unwrap();
    let data = peer_states.clone();

    thread::spawn(move || {
        loop {
            let recv_adapter = NetworkAdapter::new_incoming(&mut stream_read);
            let ps: PlayerState = recv_adapter.get_data();
            let id = ps.id;
            let mut ps_vec = data.lock().unwrap();
            if id < (*ps_vec).len() {
                (*ps_vec)[id] = ps;
            }
            else { (*ps_vec).push(ps); }
        }
    });

    let mut t1 = SystemTime::now();

    let mut mouse_pos: (f64, f64) = (0.0, 0.0);
    let mut mouse_press_pos: (f64, f64) = (0.0, 0.0);
    let mut rotate_board = false;

    let mut saved_shape: Option<Shape> = None;
    let mut last_score: u32 = 0;
    
    while window.render() {
        
        let mut states: Vec<PlayerState> = Vec::new();

        let data = peer_states.clone();
        let ps_vec = data.lock().unwrap();
        let num_peers = (*ps_vec).len();
        for i in 0..(*ps_vec).len() {
            if i != mp.id {
                states.push((*ps_vec)[i].clone());
            }
            else { states.push(my_state.clone()); }
        }

        check_target_swap(&mut my_state, &mut states, &mut saved_shape);

        let score = check_rot(&mut my_state, &mut states,
                              &mut preserved_states, &mut last_score,
                              &mut drawer, &mut window);

        drawer.draw(&mut window, &states, mp.id, score);

        for mut event in window.events().iter() {
            match event.value {
                WindowEvent::Key(code, _, Action::Press, _) => {
                    match code {
                        Key::W | Key::Up =>
                            my_state.rotate_tetromino(),
                        Key::S | Key::Down =>
                            my_state.move_down(),
                        Key::A | Key::Left =>
                            my_state.move_left(),
                        Key::D | Key::Right =>
                            my_state.move_right(),
                        Key::P =>
                            my_state.paused = !my_state.paused,
                        Key::F => {
                            drawer.anim_rot(std::f32::consts::PI / 8.0, 10);
                        },                            
                        Key::CapsLock => {
                            drawer.anim_rot(std::f32::consts::PI / -8.0, 10);
                        },
                        Key::Space =>
                            my_state.drop(),
                        Key::E =>
                            my_state.toggle_swap(1 as isize, num_peers as isize),
                        Key::C =>
                            my_state.toggle_swap(-1 as isize, num_peers as isize),
                        _ => (),
                    }
                    mp.issue_update(my_state.clone());

                    event.inhibited = true // override the default keyboard handler
                },
                WindowEvent::MouseButton(_, Action::Press, _) => {
                    rotate_board = true;
                    mouse_press_pos = mouse_pos;
                    event.inhibited = true // override the default mouse handler
                },
                WindowEvent::MouseButton(_, Action::Release, _) => {
                    rotate_board = false;
                    event.inhibited = true // override the default mouse handler
                },
                WindowEvent::CursorPos(x, y) => {
                    mouse_pos = (x, y);
                    if rotate_board {
                        drawer.orientation.prepend_rotation_mut(
                            &Vector3::new(0.0, ((mouse_pos.1 - mouse_press_pos.1) /
                                                1000.0) as f32,
                                          0.0));
                    }

                    event.inhibited = true // override the default mouse handler
                },
                _ => (),
            }
        }
        if let Ok(d) = SystemTime::now().duration_since(t1) {
            if d.as_secs() > 0.5 as u64 {
                my_state.move_down();
                t1 = SystemTime::now();
                mp.issue_update(my_state.clone());
            }
        }
    }
}

fn check_target_swap(my_state: &mut PlayerState, states: &mut Vec<PlayerState>,
                     saved_shape: &mut Option<Shape>) {
    match my_state.next_tetromino.2.clone() {
        TradeState::NoTrade => (),
        TradeState::Pending(target) =>
        {
            *saved_shape = Some(states[target].next_tetromino.0);
            match states[target].next_tetromino.2.clone() {
                TradeState::NoTrade => (),
                TradeState::Pending(id) => {
                    if id == my_state.id {
                        my_state.next_tetromino.2 = TradeState::Confirm(target);
                    }
                },
                TradeState::Confirm(id) => {
                    if id == my_state.id {
                        my_state.next_tetromino.2 = TradeState::Confirm(target);
                    }
                    else {
                        my_state.next_tetromino.2 = TradeState::NoTrade;
                    }
                },
            }
        },
        TradeState::Confirm(target) =>
        {
            match states[target].next_tetromino.2.clone() {
                TradeState::NoTrade => make_trade(my_state, saved_shape),
                TradeState::Pending(id) => {
                    if id != my_state.id {
                        *saved_shape = None;
                        my_state.next_tetromino.2 = TradeState::NoTrade;
                    }
                },
                TradeState::Confirm(id) => {
                    if id == my_state.id {
                        make_trade(my_state, saved_shape);
                    }
                    else {
                        my_state.next_tetromino.2 = TradeState::NoTrade;
                    }
                },
            }
        },
    }
}

fn make_trade(my_state: &mut PlayerState, saved_shape: &mut Option<Shape>) {
    if let &mut Some(shape) = saved_shape {
        my_state.next_tetromino.0 = shape;
        my_state.next_tetromino.1 = 0;
        my_state.next_tetromino.2 = TradeState::NoTrade;
    }
    *saved_shape = None;
}

fn check_rot(my_state: &mut PlayerState, cur_states: &mut Vec<PlayerState>,
             preserved_states: &mut Vec<PlayerState>, last_score: &mut u32,
             drawer: &mut Draw, window: &mut Window) -> u32 {

    let score = cur_states.iter().fold(0, |acc, &ref x| acc + x.score);
    
    match my_state.board_state.clone() {
        BoardState::Stable => {
            if score - *last_score >= 50 * (cur_states.len() as u32) {
                *last_score = score;
                my_state.paused = true;
                my_state.board_state = BoardState::Ready;
            }
        },
        BoardState::Ready => {
            if cur_states.iter().all(|&ref x| x.board_state != BoardState::Stable) {
                *preserved_states = cur_states.clone();
                my_state.board_state = BoardState::Confirm;
            }
        },
        BoardState::Confirm => {
            if cur_states.iter().all(|&ref x| x.board_state != BoardState::Ready) {
                drawer.anim_rot(-(std::f32::consts::PI / 2.0), 30);
                while drawer.animating() {
                    window.render();
                    drawer.draw(window, preserved_states, my_state.id, score);
                }
                my_state.rotate_board(preserved_states);
                my_state.board_state = BoardState::Stable;
                my_state.new_tetromino();
                drawer.orientation.prepend_rotation_mut(
                    &Vector3::new(0.0, std::f32::consts::PI / 2.0, 0.0));
                my_state.paused = false;
            }
        },
    }
    score
}

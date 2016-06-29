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

use game::{PlayerState, Graphics};
use multiplayer::Mp;
use networkadapter::*;

use kiss3d::window::Window;
use kiss3d::light::Light;

use nalgebra::{Vector3, Rotation};

use glfw::{Action, WindowEvent, Key};

use std::time::{Duration, SystemTime};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {

    let mut mp: Mp = Mp::new();

    let mut peer_states: Arc<Mutex<Vec<PlayerState>>> =
        Arc::new(Mutex::new(vec![PlayerState::new(0); mp.id + 1]));
    
    let mut my_state: PlayerState = PlayerState::new(mp.id);

    let mut window = Window::new("T3tropolis");
    window.set_light(Light::StickToCamera);

    let mut graphics = Graphics::new(&mut window);
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

    let mut ticks = 0;
    let mut ps_to_swap: Option<PlayerState>  = None;

    while window.render() {

        let mut states: Vec<PlayerState> = Vec::new();

        let data = peer_states.clone();
        let mut ps_vec = data.lock().unwrap();
        let num_peers = (*ps_vec).len();
        for i in 0..(*ps_vec).len() {
            if i != mp.id {
                states.push((*ps_vec)[i].clone());
            }
            else { states.push(my_state.clone()); }
        }

        if let Some(ps) = swap_if_confirmed(&mut states, mp.id) {
            ps_to_swap = Some(ps);
        }
        if let None = ps_to_swap.clone() {
            ticks += 1;
        }
        if ticks > 50 {
            if let Some(ps) = ps_to_swap.clone() {
                my_state.next_tetromino.0 = ps.next_tetromino.0;
                my_state.next_tetromino.1 = ps.next_tetromino.1;
                my_state.next_tetromino.2 = mp.id;
            }
            ticks = 0;
        }

        graphics.draw_grid(&mut window);
        graphics.draw(&mut window, &states, mp.id);

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
                        Key::Space =>
                            my_state.drop(),
                        Key::E =>
                            my_state.signal_swap(-1 as isize, num_peers as isize),
                        Key::C =>
                            my_state.signal_swap(1 as isize, num_peers as isize),
                        _ => (),
                    }
                    mp.issue_update(my_state.clone());

                    event.inhibited = true // override the default keyboard handler
                },
                WindowEvent::MouseButton(_, Action::Press, mods) => {
                    rotate_board = true;
                    mouse_press_pos = mouse_pos;
                    event.inhibited = true // override the default mouse handler
                },
                WindowEvent::MouseButton(_, Action::Release, mods) => {
                    rotate_board = false;
                    event.inhibited = true // override the default mouse handler
                },
                WindowEvent::CursorPos(x, y) => {
                    mouse_pos = (x, y);
                    if rotate_board {
                        graphics.orientation.prepend_rotation_mut(
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

fn swap_if_confirmed(states: &mut Vec<PlayerState>, my_id: usize) -> Option<PlayerState> {
    let my_ps: &PlayerState = &states[my_id].clone();
    if my_ps.next_tetromino.2 == my_id { return None; }
    let o_ps: &PlayerState = &states[my_ps.next_tetromino.2].clone();
    if o_ps.next_tetromino.2 == my_id {
        Some(o_ps.clone())
    }
    else { None }
}





 

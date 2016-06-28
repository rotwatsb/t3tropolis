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

use game::{PlayerState, Graphics, NUM_PLAYERS};
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

    let mut peer_states: Vec<Arc<Mutex<PlayerState>>> =
        vec![Arc::new(Mutex::new(PlayerState::new(0))); NUM_PLAYERS - 1];
    
    let mut my_state: PlayerState = PlayerState::new(mp.id);

    let mut window = Window::new("T3tropolis");
    window.set_light(Light::StickToCamera);

    let mut graphics = Graphics::new(&mut window);
    my_state.begin();
    
    for i in 0..NUM_PLAYERS - 1 {
        let data = peer_states[i].clone();
        let mut stream_read = mp.connection.try_clone().unwrap();
        thread::spawn(move || {
            loop {
                let recv_adapter = NetworkAdapter::new_incoming(&mut stream_read);
                let mut ps = data.lock().unwrap();
                *ps = recv_adapter.get_data();
            }
        });
    }
    
    let mut t1 = SystemTime::now();

    let mut mouse_pos: (f64, f64) = (0.0, 0.0);
    let mut mouse_press_pos: (f64, f64) = (0.0, 0.0);
    let mut rotate_board = false;

    while window.render() {

        let mut states: Vec<PlayerState> = Vec::new();
        for i in 0..NUM_PLAYERS - 1 {
            let data = peer_states[i].clone();
            let mut ps = data.lock().unwrap();
            states.push((*ps).clone());
        }
        states.push(my_state.clone());
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
                        _ => (),
                    }
                    mp.issue_update(my_state.clone());

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







 

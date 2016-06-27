use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read};
use std::str;
use std::thread;

use rustc_serialize::json;

use networkadapter::*;

use game;

pub struct Mp {
    num_players: usize,
    connection: TcpStream,
    pub peer_states: Vec<PlayerState>,
}

impl Mp {
    pub fn new(host: bool, num_players: usize) -> Mp {
        if host { create_server(); }
        Mp {
            num_players: num_players,
            connection: connect_to_server().unwrap(),
            peer_states: Vec::new(),
        }
    }

    pub fn issue_update(&mut self, ps: PlayerState) {
        /*
        let ps_enc = json::encode(&ps).unwrap();
        println!("{:?}", ps_enc);
        println!("{:?}", ps_enc.as_bytes());
        let data = Data { user: user.clone(), msg: msg };
         */
        let adapter = NetworkAdapter::new_outgoing(ps);
        send_data(&mut self.connection, adapter);
        //self.sender.write(ps_enc.as_bytes());
    }

    pub fn get_updates(&mut self) {
        
        for i in 0..self.num_players - 1 {
            let mut stream_read = self.connection.try_clone().unwrap();
            let recv_adapter = NetworkAdapter::new_incoming(&mut stream_read);
            let ps: PlayerState = recv_adapter.get_data();
            println!("Players state: {:?}", ps.clone());
            self.peer_states.push(ps);
        }
    }
}


#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct PlayerState {
    pub board: [[game::Cell; game::COLS]; game::ROWS],
    pub tetromino: (game::Shape, usize),
    pub next_tetromino: (game::Shape, usize),
    pub tetro_pos: (i8, i8),
    pub id: usize,
}

impl PlayerState {
    pub fn new(game: &game::Game) -> PlayerState {
        PlayerState {
            board: game.board,
            tetromino: game.tetromino,
            next_tetromino: game.next_tetromino,
            tetro_pos: game.tetro_pos,
            id: 2,
        }
    }
}

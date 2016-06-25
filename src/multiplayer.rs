use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read};
use std::str;
use std::thread;

use rustc_serialize::json;

use game;

pub struct Mp {
    listener: TcpListener,
    sender: TcpStream,
    peers: Vec<TcpStream>,
    pub peer_states: Vec<PlayerState>,
}

impl Mp {
    pub fn new() -> Mp {
        Mp {
            listener: TcpListener::bind("127.0.0.1:8888").unwrap(),
            sender: TcpStream::connect("127.0.0.1:8888").unwrap(),
            peers: Vec::new(),
            peer_states: Vec::new(),
        }
    }

    pub fn issue_update(&mut self, ps: &PlayerState) {
        let ps_enc = json::encode(&ps).unwrap();
        self.sender.write(ps_enc.as_bytes());
    }

    pub fn open_peers(&mut self) {
        for stream in self.listener.incoming().take(1) {
            match stream {
                Ok(stream) => {
                    self.peers.push(stream);
                }
                Err(e) => { }
            }
        }
    }

    pub fn get_updates(&mut self) {
        self.peer_states.drain(..);
        println!("{}", self.peers.len());
        let mut buf: [u8; 1338] = [0; 1338];
        for ref mut stream in self.peers.iter_mut() {
            println!("HOOOOOOOOOOOOOOOO");
            stream.read(&mut buf);
            let mut ps: PlayerState =
                json::decode(str::from_utf8(&buf[..]).unwrap()).unwrap();
            println!("{:?}", ps);
            self.peer_states.push(ps);
        }
    }
}


#[derive(RustcDecodable, RustcEncodable, Debug)]
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

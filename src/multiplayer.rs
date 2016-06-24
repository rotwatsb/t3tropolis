use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read};
use std::str;

use rustc_serialize::json;

use game;

pub struct Mp {
    listener: TcpListener,
    sender: TcpStream,
    peers: Vec<TcpStream>,
}

impl Mp {
    pub fn new() -> Mp {
        Mp {
            listener: TcpListener::bind("127.0.0.1:8888").unwrap(),
            sender: TcpStream::connect("127.0.0.1:8888").unwrap(),
            peers: Vec::new(),
        }
    }

    pub fn issue_update(&mut self, ps: &PlayerState) {
        let ps_enc = json::encode(&ps).unwrap();
        self.sender.write(ps_enc.as_bytes());
        println!("{:?}", ps_enc.as_bytes());
    }

    pub fn get_updates(&mut self) -> Vec<PlayerState> {
        self.get_streams();
        let mut v = Vec::new();
        let mut buf: [u8; 1338] = [0; 1338];
        
        for ref mut stream in self.peers.iter_mut() {
            println!("HEY Pre");

            stream.read(&mut buf);
            let mut ps: PlayerState =
                json::decode(str::from_utf8(&buf[..]).unwrap()).unwrap();
            println!("HEY Post");

            v.push(ps);
        }
        v
    }

    fn get_streams(&mut self) {
        println!("HEY Pre get stream");

        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.peers.push(stream);
                    break;
                }
                Err(e) => { /* connection failed */ }
            }
        }
        println!("HEY Post get stream");
    }
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct PlayerState {
    board: [[game::Cell; game::COLS]; game::ROWS],
    tetromino: (game::Shape, usize),
    next_tetromino: (game::Shape, usize),
    tetro_pos: (i8, i8),
    id: usize,
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

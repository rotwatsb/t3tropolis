use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read};
use std::str;
use std::thread;

use networkadapter::*;

use bincode::rustc_serialize::{encode, decode};
use rustc_serialize::{Encodable, Decodable};

use game::{PlayerState};

pub struct Mp {
    pub connection: TcpStream,
}

impl Mp {
    pub fn new() -> Mp {
        to_host_or_not();
        Mp {
            connection: connect_to_server().unwrap(),
        }
    }

    pub fn issue_update(&mut self, ps: PlayerState) {
        let adapter = NetworkAdapter::new_outgoing(ps);
        send_data(&mut self.connection, adapter);
    }
}





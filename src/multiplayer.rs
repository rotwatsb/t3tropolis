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
    pub id: usize,
}

impl Mp {
    pub fn new() -> Mp {
        to_host_or_not();
        let cnx = connect_to_server().unwrap();
        let mut stream_read = cnx.try_clone().unwrap();
        let recv_adapter = NetworkAdapter::new_incoming(&mut stream_read);
        let id: usize = recv_adapter.get_data();
        Mp {
            connection: cnx,
            id: id,
        }
    }

    pub fn issue_update(&mut self, ps: PlayerState) {
        let adapter = NetworkAdapter::new_outgoing(ps);
        send_data(&mut self.connection, adapter);
    }
}





use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io;
use std::io::{stdout, stdin};
use std::io::prelude::*;
use std::marker::PhantomData;
use bincode::SizeLimit;
use bincode::rustc_serialize::{encode, decode};
use rustc_serialize::{Encodable, Decodable};

use playerstate::PlayerState;

#[derive(Debug)]
pub struct NetworkAdapter<T> 
    where T: Encodable + Decodable
{
    pub length: u32,
    pub data: Vec<u8>,
    phantom: PhantomData<T>
}

impl<T> NetworkAdapter<T> 
    where T: Encodable + Decodable
{
    pub fn new_incoming(stream: &mut TcpStream) -> Self {
        let length: u32 = read_length(stream);

        assert!(length > 4);
        let read_length = (length as usize) - 4;
        let mut data: Vec<u8> = vec![0; read_length];
        stream.read_exact(&mut data.as_mut_slice()).unwrap();
        NetworkAdapter {
            length: length,
            data: data,
            phantom: PhantomData::<T>
        }
    }

    pub fn new_outgoing(data: T) -> Self {
        let mut data = encode(&data, SizeLimit::Infinite).unwrap();
        let length = (4 + data.len()) as u32;
        if length < 5 {panic!("data can't be zero")}
        let mut len = encode(&length, SizeLimit::Infinite).unwrap();
        len.append(&mut data);
        let data = len;
        NetworkAdapter {
            length: length,
            data: data,
            phantom: PhantomData::<T>
        }
    }

    pub fn get_data(&self) -> T {
        decode(&self.data[..]).unwrap()
    }

}

fn read_length(stream: &mut TcpStream) -> u32 {
    let mut buf: [u8; 4] = [0; 4];
    stream.read_exact(&mut buf).unwrap();
    decode(&buf).unwrap()
}

enum NetworkEvent {
    NewConnection(usize, TcpStream),
    NewMessage(usize, PlayerState),
}

fn handle_stream(mut stream: TcpStream, id: usize, tx: Sender<NetworkEvent>) {
    thread::spawn(move|| {
        loop {
            let recv_adapter = NetworkAdapter::new_incoming(&mut stream);
            let data: PlayerState = recv_adapter.get_data();
            tx.send(NetworkEvent::NewMessage(id, data));
        }
    });
}


pub fn create_server(host_port: String) {
    let (tx, rx): (Sender<NetworkEvent>, Receiver<NetworkEvent>) = mpsc::channel();
    let addr: String = "0.0.0.0:".to_string() + &host_port;
    println!("Creating a servers at {}", addr);
    let listener = TcpListener::bind(&addr as &str).unwrap();
    // handle incoming connections
    thread::spawn(move|| {
        let mut id = 0;
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let tx = tx.clone();
                    tx.send(
                        NetworkEvent::NewConnection(
                            id, stream.try_clone().unwrap()));
                    handle_stream(stream, id, tx);
                    id += 1;
                },
                Err(e) => println!("{:?}", e)
            }
        }
    });

    thread::spawn(move|| {
        let mut conns: Vec<TcpStream> = vec![];
        loop {
            // handle sending pp to all streams
            let event = rx.recv().unwrap();
            match event {
                NetworkEvent::NewConnection(id, stream) => {
                    conns.push(stream);
                    let adapter = NetworkAdapter::new_outgoing(id);
                    conns[id].write(adapter.data.as_slice());
                },
                NetworkEvent::NewMessage(id, data) => {
                    let adapter = NetworkAdapter::new_outgoing(data);
                    for (idx, ref mut conn) in conns.iter_mut().enumerate() {
                        if id != idx {
                            conn.write_all(adapter.data.as_slice());
                        }
                    }
                }
            }
        }
    });
}

pub fn connect_to_server() -> io::Result<TcpStream> {
    print!("Connect to: ");
    let addr = &get_input();
    println!("Connecting to server {}!", addr);
    let stream = try!(TcpStream::connect(&addr as &str));
    Ok(stream)
}

pub fn send_data<T>(stream: &mut TcpStream, 
                    adapter: NetworkAdapter<T>) 
    where T: Encodable + Decodable {
    stream.write_all(adapter.data.as_slice());
}

pub fn to_host_or_not() {
    println!("Would you like to host? (y/n) ");
    let mut input = String::new();
    print!("> ");
    let _ = stdout().flush();
    let _ = stdin().read_line(&mut input);
    let input = input.trim();
    match input {
        "y" => {
            print!("Host port: ");
            let port = get_input();
            create_server(port);
        }
        _ => return
    }
}

fn get_input() -> String {
    print!("> ");
    let mut input = String::new();
    let _ = stdout().flush();
    let _ = stdin().read_line(&mut input);
    input.trim().to_string()
}

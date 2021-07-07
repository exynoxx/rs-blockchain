use bincode::{deserialize, serialize};
use rand::Rng;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::env;
use std::io::Read;
use std::io::Write;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::net::TcpListener;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;

use crate::blockchain::{Block, BlockChain, SignedBlock, SignedTransaction};

pub struct Network {
    pub connections: Vec<TcpStream>,
    pub msg_received: HashMap<usize, Message>,
    pub local_address: SocketAddr,
    pub incoming_connections: Receiver<TcpStream>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub id: usize,
    pub typ: usize,
    pub transaction: Option<SignedTransaction>,
    pub block: Option<SignedBlock>,
}


pub fn new() -> Network {
    let (_, rx) = mpsc::channel();

    return Network {
        connections: Vec::new(),
        msg_received: HashMap::new(),
        local_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
        incoming_connections: rx,
    };
}


impl Network {
    pub fn setup(&mut self) {

        // CONNECT TO PEER GIVEN IN ARGS, IF NONE: SKIP
        let args = env::args().skip(1).collect::<Vec<_>>();
        if args.len() >= 1 {
            let addr: String = args[0].parse().unwrap();
            match TcpStream::connect(addr) {
                Ok(stream) => {
                    stream.set_nonblocking(true);
                    self.connections.push(stream);
                }
                Err(t) => println!("Couldn't connect to peer ({})", t)
            }
        }

        //SPAWN THREAD THAT LISTENS FOR INCOMING CONNECTIONS, streams ARE SEND TO RX
        let (tx, rx) = mpsc::channel();
        self.incoming_connections = rx;

        let (addrSender, addrReceiver) = mpsc::channel();

        thread::spawn(move || {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            println!("Listening for incoming connections on addr {:?}", listener.local_addr().unwrap());
            addrSender.send(listener.local_addr().unwrap());

            for stream in listener.incoming() {
                let stream = stream.unwrap();
                tx.send(stream);
            }
        });
        match addrReceiver.recv() {
            Ok(v) => self.local_address = v,
            Err(e) => println!("error"),
        }
    }


    // THESE 2 METHODS COME TOGETHER
    pub fn listen_connection(&mut self) {
        match self.incoming_connections.try_recv() {
            Ok(stream) => {
                println!("mpsc: got stream from {}", stream.peer_addr().unwrap());
                stream.set_nonblocking(true);
                self.connections.push(stream);
                self.flood(&Message { id: 0, typ: 0, transaction: None, block: None });
            }
            Err(t) => ()
        }
    }
    pub fn listen_data(&mut self, blockchain: &mut BlockChain, data_handler: fn(&mut BlockChain, &Message)) {
        //receive data (non blocking) on each stream
        const BUFFERSIZE: usize = 1024;
        let mut buffer = [0u8; BUFFERSIZE];

        let mut tobe_redistributed: Vec<[u8; BUFFERSIZE]> = vec![];

        for (i, mut stream) in self.connections.iter().enumerate() {
            match stream.read(&mut buffer) {
                Ok(_) => {
                    let msg: Message = deserialize(&buffer).unwrap();
                    if !self.msg_received.contains_key(&msg.id) {
                        (data_handler)(blockchain, &msg); //some method supplied in main.rs
                        self.msg_received.insert(msg.id, msg);
                        tobe_redistributed.push(buffer);
                    }
                }
                Err(e) => ()
            }
        }

        for buffer in tobe_redistributed.iter() {
            self.reflood(buffer)
        }
    }

    pub fn flood(&mut self, msg: &Message) {
        let raw_data = serialize(msg).unwrap();

        for (i, mut stream) in self.connections.iter().enumerate() {
            stream.write(&raw_data);
        }
    }

    pub fn flood_transaction(&mut self, data: &SignedTransaction) {
        let mut rng = rand::thread_rng();

        let msg = Message {
            id: rng.gen::<usize>(),
            typ: 1,
            transaction: Some(data.clone()),
            block: None,
        };
        let raw_data = serialize(&msg).unwrap();

        for (i, mut stream) in self.connections.iter().enumerate() {
            stream.write(&raw_data);
        }
    }

    pub fn flood_block(&mut self, data: &SignedBlock) {
        let mut rng = rand::thread_rng();

        let msg = Message {
            id: rng.gen::<usize>(),
            typ: 2,
            transaction: None,
            block: Some(data.clone()),
        };
        let raw_data = serialize(&msg).unwrap();

        for (i, mut stream) in self.connections.iter().enumerate() {
            stream.write(&raw_data);
        }
    }
    pub fn reflood(&mut self, data: &[u8]) {
        for (i, mut stream) in self.connections.iter().enumerate() {
            stream.write(data);
        }
    }

    pub fn single(&mut self, i: usize, data: &[u8]) {
        self.connections[i].write(data);
    }
}





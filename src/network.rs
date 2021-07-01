use bincode::{deserialize, serialize};

use std::env;
use std::io::Read;
use std::io::Write;
use std::net::{Ipv4Addr, SocketAddr, TcpStream, IpAddr};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;
use std::collections::HashMap;
use rand::Rng;

pub struct Network {
    pub connections: Vec<TcpStream>,
    pub data_handler: fn(&Network, &String),
    //callback when the network receives data.
    pub msg_received: HashMap<u64, String>,
    pub msg_counter: u64,
    //remove me
    pub local_address: SocketAddr,
}

struct Message {
    id: u64,
    content: [u8],
}

pub fn new(handle: fn(&Network, &String)) -> Network {
    return Network {
        connections: Vec::new(),
        data_handler: handle,
        msg_received: HashMap::new(),
        msg_counter: 0,
        local_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
    };
}


impl Network {
    pub fn setup(&mut self) -> mpsc::Receiver<TcpStream> {

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
        return rx;
    }


    // THESE 2 METHODS COME TOGETHER
    pub fn listen_connection(&mut self, rx: &mpsc::Receiver<TcpStream>) {
        match rx.try_recv() {
            Ok(stream) => {
                println!("mpsc: got stream from {}", stream.peer_addr().unwrap());
                stream.set_nonblocking(true);
                self.connections.push(stream);
                self.flood(&"connected".to_string());
            }
            Err(t) => ()
        }
    }
    pub fn listen_data(&mut self) {
        //receive data (non blocking) on each stream
        const buffersize: usize = 1024;
        let mut buffer = [0u8; buffersize];
        let mut tobe_redistributed: Vec<[u8; buffersize]> = vec![];

        for (i, mut stream) in self.connections.iter().enumerate() {
            match stream.read(&mut buffer) {
                Ok(_) => {
                    let (counter, msg): (u64, String) = deserialize(&buffer).unwrap();
                    if !self.msg_received.contains_key(&counter) {
                        (self.data_handler)(&self, &msg); //some method supplied in main.rs
                        self.msg_received.insert(counter, msg);
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

    // -------------------------------------

    pub fn event_loop(&mut self, rx: &mpsc::Receiver<TcpStream>) {
        // ABOVE 2 METHODS
        self.listen_connection(&rx);
        self.listen_data();
    }

    pub fn flood(&mut self, data: &String) {
        let mut rng = rand::thread_rng();
        let raw_data = serialize(&(rng.gen::<u64>(), data)).unwrap();

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





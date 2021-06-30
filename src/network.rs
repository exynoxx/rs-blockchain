use std::env;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::{TcpStream, Ipv4Addr};
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;

use bincode::{deserialize, serialize};

pub struct Network {
    pub connections: Vec<TcpStream>,
    pub data_handler: fn(&[u8]), //callback when the network receives data.
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
                    self.connections.push(stream)
                }
                Err(t) => println!("Couldn't connect to peer ({})", t)
            }
        }

        //SPAWN THREAD THAT LISTENS FOR INCOMING CONNECTIONS, streams ARE SEND TO RX
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            println!("Listening for incoming connections on addr {:?}", listener.local_addr().unwrap());

            for stream in listener.incoming() {
                let stream = stream.unwrap();
                tx.send(stream);
            }
        });

        return rx;
    }


    // THESE 2 METHODS COME TOGETHER
    pub fn listen_connection(&mut self, rx: &mpsc::Receiver<TcpStream>) {
        match rx.try_recv() {
            Ok(stream) => {
                println!("mpsc: got stream from {}", stream.peer_addr().unwrap());
                stream.set_nonblocking(true);
                self.connections.push(stream);
            }
            Err(t) => ()
        }
        self.flood(&serialize(&("init", 0)).unwrap());
    }
    pub fn listen_data(&mut self) {
        //receive data (non blocking) on each stream
        const buffersize: usize = 1024;
        let mut buffer = [0u8; buffersize];

        for (i, mut stream) in self.connections.iter().enumerate() {
            match stream.read(&mut buffer) {
                Ok(_) => (self.data_handler)(&buffer), //some method supplied in main.rs
                Err(e) => ()
            }
        }
    }

    // -------------------------------------

    pub fn event_loop(&mut self, rx: &mpsc::Receiver<TcpStream>) {
        loop {
            // ABOVE 2 METHODS
            self.listen_connection(&rx);
            self.listen_data();
        }
    }

    pub fn flood(&mut self, data: &[u8]) {
        for (i, mut stream) in self.connections.iter().enumerate() {
            stream.write(data);
        }
    }

    pub fn single(&mut self, i:usize, data: &[u8]){
        self.connections[i].write(data);
    }
}





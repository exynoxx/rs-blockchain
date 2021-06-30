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
    pub connections: Vec<Arc<Mutex<TcpStream>>>,
    pub callback: fn(&[u8]),
}

//let (send, recv) = mpsc::channel();

impl Network {
    pub fn setup(&mut self) {
        let (tx, rx) = mpsc::channel();
        handle_loop(rx);

        // CONNECT TO PEER GIVEN IN ARGS, IF NONE: SKIP
        let args = env::args().skip(1).collect::<Vec<_>>();
        if args.len() >= 1 {
            let addr: String = args[0].parse().unwrap();
            match TcpStream::connect(addr) {
                Ok(stream) => {
                    tx.send(stream);
                }
                Err(t) => println!("Couldn't connect to peer ({})", t)
            }
        }


        //self.flood("greetings".as_bytes());

        //LISTEN FOR CONNECTIONS TO US
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        println!("Listening for incoming connections on addr {:?}", listener.local_addr().unwrap());

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            tx.send(stream);

            /*self.connections.push(Arc::new(Mutex::new(stream)));
            let last_stream_ptr = self.connections.last().unwrap().clone();
            thread::spawn(move || handle_connection(&mut *last_stream_ptr.lock().unwrap()));*/

            /*let (send, recv) = mpsc::channel();
            self.connections.push(send);
            thread::spawn(move || handle_connection(stream, recv));*/
        }
    }
}


/*let last_stream_ptr = self.connections.last().unwrap().clone();
                    thread::spawn(move || handle_connection(&mut *last_stream_ptr.lock().unwrap()));
                    /*let (send, recv) = mpsc::channel();
                    self.connections.push(send);
                    thread::spawn(move || handle_connection(stream, recv));*/*/
//WHEN a STREAM IS OPENED, THIS IS CALLED


fn handle_loop(rx: mpsc::Receiver<TcpStream>) {
    println!("---------");

    thread::spawn(move || {
        println!("tcp thread started");
        let mut connections: Vec<TcpStream> = vec![];
        const buffersize:usize = 1024;
        let mut buffers: Vec<[u8;buffersize]> = vec![];

        let mut i = 0;

        loop {
            //mpsc sends new connection (stream): store it
            match rx.try_recv() {
                Ok(stream) => {
                    println!("mpsc: got stream from {}", stream.peer_addr().unwrap());
                    stream.set_nonblocking(true);
                    connections.push(stream);
                    buffers.push([0;buffersize]);
                }
                Err(t) => ()
            }

            i+=1;
            if i % 10000000 == 0{
                println!("sending");
                for (i,mut stream) in connections.iter().enumerate() {
                    stream.write(&serialize(&"greet").unwrap());
                }
            }


            //receive data (non blocking) on each stream
            for (i,mut stream) in connections.iter().enumerate() {
                match stream.read(&mut buffers[i]) {
                    Ok(_) => handle_data(&buffers[i], &connections),
                    Err(e) =>()
                }
            }
        }
    });
}

fn handle_data(data:&[u8], connections: &Vec<TcpStream>){
    let d:String = deserialize(data).unwrap();
    println!("handling data {:?}", d);
}



//use std::io::{Listener, Acceptor};
//https://users.rust-lang.org/t/multi-threading-tcp-writing-reading-on-tcpstream/10558
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;
use std::cell::RefCell;
use std::thread;

fn main() {
        let listener = TcpListener::bind("127.0.0.1:9123").unwrap();
        println!("listening started, ready to accept");
        for stream in listener.incoming() {
//            thread::spawn(move || { 
                let mut wr_stream = stream.unwrap();
                let mut rd_stream = wr_stream.try_clone().unwrap();
//                wr_stream.write(b"Hello World\n").unwrap();
//                wr_stream.flush();
                let mut line = String::new();
//                loop {
//                    match rd_stream.read_to_string(&mut line) {
                    match rd_stream.read(&mut line) {

                        Ok(_) => {
                            println!("client says {}", line);
                        }
                        Err(error) => {
                            eprintln!("Error {}", error);
                        }
                        
                    }
                //}
               
                
 //           });
        }
        
}

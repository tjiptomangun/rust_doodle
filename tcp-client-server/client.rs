//https://github.com/valenting/rust-learning/blob/master/tcp-client-server/client.rs
//https://users.rust-lang.org/t/how-to-write-a-simple-tcp-client-and-server/3712

use std::io::prelude::*;
use std::net::TcpStream;
use std::io::{self, Read, Write, Error, stdin, stdout};
use std::process;
use std::{thread, time};

fn main (){
    let o_stream = TcpStream::connect("127.0.0.1:9123");

    let mut conn_stream = match o_stream {
        Ok(s) => s,
        Err(error)=> {
            eprintln!("No Connection {}", error);
            process::exit(1);
        }
    };

    let mut resp = String::new();
    conn_stream.read_to_string(&mut resp).unwrap();
    conn_stream.flush();
    println!("hello server says {}", resp);
    //stdout.flush().unwrap();
//    let bf = BufReader::new(conn_stream);

    let mut line = String::new();
    let ten_millis = time::Duration::from_millis(10);

    loop {
        thread::sleep(ten_millis);
        /*
        match stdin().read_line(&mut line){
            Ok(_) => {
                conn_stream.write(line.as_bytes());
            }
            Err(error) => {
                eprintln!("error: {}", error);
                process::exit(1); 
            }
        }
        */
    } 
}

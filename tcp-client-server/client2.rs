//https://github.com/valenting/rust-learning/blob/master/tcp-client-server/client.rs
//https://users.rust-lang.org/t/how-to-write-a-simple-tcp-client-and-server/3712
//https://gist.github.com/postmodern/0cf7cf8ec008c3713ef76cc6f4b3ffc1
//https://users.rust-lang.org/t/multi-threading-tcp-writing-reading-on-tcpstream/10558

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
    
    let mut input_stream = conn_stream.try_clone().unwrap();

    let handler = thread::spawn(move || {
        let mut client_buffer = [0u8, 1024];
        loop {
            match input_stream.read(&mut client_buffer) {
                Ok(n) => {
                    if n == 0 {
                        process::exit(1);
                    }
                    else {
                        stdout().write(&client_buffer).unwrap();
                        stdout().flush().unwrap();
                    }
                }
                Err(err) => {
                    eprintln!("error {} ", err);
                }
            }
        }
    });


    let output_stream = &mut conn_stream;
    let mut user_buffer = String::new();

    loop {
            io::stdin().read_line(&mut user_buffer).unwrap();

            output_stream.write(user_buffer.as_bytes()).unwrap();
            output_stream.flush().unwrap();
    }

    handler.join();
}

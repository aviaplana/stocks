use std::{
    net::{TcpListener, TcpStream},
    thread,
    sync::mpsc::{channel, Receiver},
    io::{Read, Write, Error},
    str,
};

pub fn launch_tcp_server() ->  Receiver<String>{
    let listener = TcpListener::bind("0.0.0.0:8888")
        .expect("Could not bind");

    let (tx, rx) = channel::<String>();
    
    let tx_th = tx.clone();
    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Err(e) => eprintln!("Failed: {}", e),
                Ok(mut stream) => { 
                    let mut buf = [0; 512];
                    let mut bytes_read = 0;
                    while {
                        bytes_read = stream.read(&mut buf).unwrap();
                        bytes_read != 0
                    } {
                        let operation = str::from_utf8(&buf[..bytes_read]).unwrap();
                        println!("Got: {}", operation);
                        tx_th.send(operation.to_string()).unwrap();                        
                    }
                } 
            }
        }
    });

    rx
}
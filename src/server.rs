use std::{
    net::{TcpListener, TcpStream},
    io::{Read, Write},
    str,
};
use crossbeam_channel::{
    unbounded,
    bounded,
    Sender,
    Receiver, 
};
use crate::{Job, Operation, ToBytes};
use tokio;

pub fn launch_tcp_server() ->  Receiver<(Vec<u8>, Sender<Vec<u8>>)> {
    let listener = TcpListener::bind("0.0.0.0:8888")
        .expect("Could not bind");

    let (tx, rx) = unbounded(); // Sends / recives  Sender<Vec<u8>

    listen_socket(listener, tx); 

    rx
}

fn listen_socket(listener: TcpListener, tx: Sender<(Vec<u8>, Sender<Vec<u8>>)>) {
    tokio::spawn(async move {
        let mut connection_id: u32 = 0;

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => { 
                    let tx_task = tx.clone();
                    process_connection(connection_id, stream, tx_task);
                    connection_id = connection_id + 1;
                },
                Err(e) => eprintln!("Failed: {}", e)
            }
        }
    });
}

fn process_connection(connection_id: u32, mut stream: TcpStream, tx: Sender<(Vec<u8>, Sender<Vec<u8>>)>) {
    tokio::task::spawn_blocking(move || {
        println!("[SERVER] New job from connection id {}", connection_id);
        let mut buf = [0; 512];
        let mut bytes_read = 0;

        while {
            bytes_read = stream.read(&mut buf).unwrap();
            bytes_read != 0
        } {
            let message = str::from_utf8(&buf[..bytes_read])
                .unwrap()
                .trim_end_matches('\n'); //Removes tailing new line;
            println!("[SERVER] Received message: {}", message);

            let operation = Operation::from(message.to_string());

            if operation != Operation::Error {
                let rx = send_job(connection_id, &operation, &tx);
                wait_and_process_response(connection_id, &rx, &stream);
            } else {
                eprintln!("[SERVER] Operation not valid.")
            }
        }
        
        println!("[SERVER] Connection {} closed", connection_id);
    });

}

fn send_job(connection_id: u32, 
        operation: &Operation, 
        tx: &Sender<(Vec<u8>, Sender<Vec<u8>>)>) -> Receiver<Vec<u8>> {
    let serialized_op = operation.to_bytes();
                
    let job = Job {
        id: connection_id, 
        payload: serialized_op
    };
    println!("[SERVER] Sending job {:?}", job);
    let (tx2, rx2) = bounded(1);
    tx.send((job.to_bytes(), tx2)).unwrap();
    rx2
}

fn wait_and_process_response(connection_id: u32, rx: &Receiver<Vec<u8>>, mut stream: &TcpStream) {
    loop {
        let response = rx.recv().unwrap();
        println!("[SERVER] Raw response: {:?}", response);

        let resp_job= Job::from(&response);
        if resp_job.id == connection_id {
            let payload_arr = resp_job.payload.as_slice();
            let to_str = str::from_utf8(payload_arr).unwrap();
            println!("[SERVER] Deserialized: {}", to_str);
            stream.write(payload_arr).unwrap();
            break;
        } 
    }
}
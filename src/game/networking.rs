use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    time::Duration,
};

pub(crate) struct Connection {
    pub stream: TcpStream,
}

impl Connection {
    pub fn new_server(address: &str) -> Self {
        let listener = TcpListener::bind(address).expect("Failed to bind address");

        listener
            .set_nonblocking(true)
            .expect("Failed to set nonblocking to true");

        loop {
            match listener.accept() {
                Ok((stream, address)) => {
                    println!("Client connected: {}", address);

                    stream
                        .set_nonblocking(true)
                        .expect("Failed to set nonblocking to true");
                    return Connection { stream };
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    println!("Waiting for client to connect...");
                    std::thread::sleep(Duration::from_secs(1));
                }
                Err(e) => {
                    panic!("Error: {}", e);
                }
            }
        }
    }

    pub fn new_client(address: &str) -> Self {
        let stream = TcpStream::connect(address).expect("Failed to connect to host");

        stream
            .set_nonblocking(true)
            .expect("Failed to set non-blocking to true");

        Connection { stream }
    }

    pub fn read(&mut self) -> Vec<u8> {
        let mut buf = [0u8; 512];

        match self.stream.read(&mut buf) {
            Ok(len) => buf[0..len].to_vec(),
            Err(err) => {
                println!("Error: {}", err);
                Vec::new()
            }
        }
    }

    pub fn write(&mut self, buf: Vec<u8>) {
        self.stream
            .write_all(&buf)
            .expect("Failed to write to stream");
    }
}

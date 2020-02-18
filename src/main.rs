#![feature(io)]
#![feature(path)]

use std::os::unix::net::{UnixStream, UnixListener};
use std::path::Path;

fn main() {
    let socket = Path::new("plugin.sock");

    // Delete old socket if necessary
    if socket.exists() {
        std::fs::remove_file(&socket)?;
        Ok(())
    }

    // Bind to socket
    let listener = match UnixListener::bind(&socket) {
        Err(_) => panic!("failed to bind socket"),
        Ok(listener) => listener,
    };

    println!("Server started, waiting for clients");

    // Iterate over clients, blocks if no client available
    for mut stream in listener.incoming() {
        println!("Client said: {}", stream.read_to_string().unwrap());
    }
}

use std::os::unix::net::{UnixStream, UnixListener};
use std::path::Path;

fn main() {
    let socket = Path::new("plugin.sock");

    // Delete old socket if necessary
    if socket.exists() {
        std::fs::remove_file(&socket);
    }

    // Bind to socket
    let listener = match UnixListener::bind(&socket) {
        Err(_) => panic!("failed to bind socket"),
        Ok(listener) => listener
    };

    println!("Server started, waiting for clients");

    // Iterate over clients, blocks if no client available
    for mut stream in listener.incoming() {
        let mut request = String::new();
        stream.read_to_string(&mut request)?;
        println!("Client asks: {}", request);
        stream.write_all(b"hello world")?;
    }
}

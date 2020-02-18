use std::os::unix::net::{UnixStream, UnixListener};
use std::path::Path;

fn main() -> std::io::Result<()> {
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
    for streamResult in listener.incoming() {
        match streamResult {
            Ok(stream) => {
                let mut request = String::new();
                stream.read_to_string(&mut request)?;
                println!("Client asks: {}", request);
                stream.write_all(b"hello world")?;
            }
            Err(err) => {
                // Delete socket on error
                if socket.exists() {
                    std::fs::remove_file(&socket);
                }
                break;
            }
        }
    }
    Ok(()) // -> std::io::Result<()>
}

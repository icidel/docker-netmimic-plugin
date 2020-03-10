use std::io::Read;
use std::io::Write;
use std::os::unix::net::{UnixStream, UnixListener};
use std::path::Path;

fn main() -> std::io::Result<()> {
    let action_regex = Regex::new(
    r#"POST \/(?<action>Plugin\.Activate|NetworkDriver\.[^.]+)"#
    ).unwrap();
    
    let socket = Path::new("plugin.sock");

    // Delete old socket if necessary
    if socket.exists() {
        std::fs::remove_file(&socket);
    }

    // Bind to socket
    let listener = match UnixListener::bind(&socket) {
        Err(_) => panic!("failed to bind socket [plugin.sock]"),
        Ok(listener) => listener
    };

    println!("Server started, waiting for clients on socket [plugin.sock]");

    // Iterate over clients, blocks if no client available
    for streamResult in listener.incoming() {
        match streamResult {
            Ok(mut stream) => {
                let mut request = String::new();
                stream.read_to_string(&mut request)?;
                println!("Client asks: {}", request);
                let response_body = match request {
                    if (request.starts_with("POST /Plugin.Activate")) => "",
                    if (request.starts_with("POST /NetworkPlugin.AllocateNetwork")) => "",
                    if (request.starts_with("POST /NetworkPlugin.FreeNetwork")) => "",
                    if (request.starts_with("POST /NetworkPlugin.CreateNetwork")) => "",
                    if (request.starts_with("POST /NetworkPlugin.DeleteNetwork")) => "",
                    if (request.starts_with("POST /NetworkPlugin.CreateEndpoint")) => "",
                    if (request.starts_with("POST /NetworkPlugin.EndpointOperInfo")) => "",
                    if (request.starts_with("POST /NetworkPlugin.DeleteEndpoint")) => "",
                    if (request.starts_with("POST /NetworkPlugin.Join")) => "",
                    if (request.starts_with("POST /NetworkPlugin.Leave")) => "",
                    if (request.starts_with("POST /NetworkPlugin.DiscoverNew")) => "",
                    if (request.starts_with("POST /NetworkPlugin.DiscoverDelete")) => "",
                    if (request.starts_with("POST /NetworkPlugin.ProgramExternalConnectivity")) => "",
                    if (request.starts_with("POST /NetworkPlugin.RevokeExternalConnectivity")) => "",
                    .. => return Err(format!("Unknown action {}", request)),
                };
                
                let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", response_body).as_bytes();
                stream.write_all(response)?;
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

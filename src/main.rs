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
                    Some(req) if (req.starts_with("POST /Plugin.Activate")) => "",
                    Some(req) if (req.starts_with("POST /NetworkPlugin.AllocateNetwork")) => "",
                    Some(req) if (req.starts_with("POST /NetworkPlugin.FreeNetwork")) => "",
                    Some(req) if (req.starts_with("POST /NetworkPlugin.CreateNetwork")) => "",
                    Some(req) if (req.starts_with("POST /NetworkPlugin.DeleteNetwork")) => "",
                    Some(req) if (req.starts_with("POST /NetworkPlugin.CreateEndpoint")) => "",
                    Some(req) if (req.starts_with("POST /NetworkPlugin.EndpointOperInfo")) => "",
                    Some(req) if (req.starts_with("POST /NetworkPlugin.DeleteEndpoint")) => "",
                    Some(req) if (req.starts_with("POST /NetworkPlugin.Join")) => "",
                    Some(req) if (req.starts_with("POST /NetworkPlugin.Leave")) => "",
                    Some(req) if (req.starts_with("POST /NetworkPlugin.DiscoverNew")) => "",
                    Some(req) if (req.starts_with("POST /NetworkPlugin.DiscoverDelete")) => "",
                    Some(req) if (req.starts_with("POST /NetworkPlugin.ProgramExternalConnectivity")) => "",
                    Some(req) if (req.starts_with("POST /NetworkPlugin.RevokeExternalConnectivity")) => "",
                    Some(req) => return Err(format!("Unknown action {}", req)),
                    None => return Err("No action"),
                }
                
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

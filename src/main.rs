use std::fmt; // for custom error handling formatting
use std::io::Read;
use std::io::Write;
use std::os::unix::net::{UnixStream, UnixListener};
use std::path::Path;

#[derive(Debug)] // derive std::fmt::Debug on RequestHandling
struct RequestHandling {
    code: usize,
    request: String,
}

impl fmt::Display for RequestHandling {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = match self.code {
            404 => format!("No match found for request: {}",self.request),
            _ => format!("Unknown error code: {}",self.code),
        };

        write!(f, "{}", err_msg)
    }
}

fn main() -> std::io::Result<()> {
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
                    req if (req.starts_with("POST /Plugin.Activate")) => "",
                    req if (req.starts_with("POST /NetworkPlugin.AllocateNetwork")) => "",
                    req if (req.starts_with("POST /NetworkPlugin.FreeNetwork")) => "",
                    req if (req.starts_with("POST /NetworkPlugin.CreateNetwork")) => "",
                    req if (req.starts_with("POST /NetworkPlugin.DeleteNetwork")) => "",
                    req if (req.starts_with("POST /NetworkPlugin.CreateEndpoint")) => "",
                    req if (req.starts_with("POST /NetworkPlugin.EndpointOperInfo")) => "",
                    req if (req.starts_with("POST /NetworkPlugin.DeleteEndpoint")) => "",
                    req if (req.starts_with("POST /NetworkPlugin.Join")) => "",
                    req if (req.starts_with("POST /NetworkPlugin.Leave")) => "",
                    req if (req.starts_with("POST /NetworkPlugin.DiscoverNew")) => "",
                    req if (req.starts_with("POST /NetworkPlugin.DiscoverDelete")) => "",
                    req if (req.starts_with("POST /NetworkPlugin.ProgramExternalConnectivity")) => "",
                    req if (req.starts_with("POST /NetworkPlugin.RevokeExternalConnectivity")) => "",
                    _ => return RequestHandling {
                        code: 404,
                        request: &request,
                    },
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

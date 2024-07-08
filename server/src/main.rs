use std::{io, thread};
use std::io::{BufReader, BufWriter, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, mpsc, Mutex, RwLock};
use std::sync::mpsc::{Receiver, Sender};
use shared::Framer;
use shared::packet::{Command, Response};
use shared::protocol::LengthDelineated;

pub struct ResponseFuture(Receiver<Response>);

impl ResponseFuture {
    pub fn wait(self) -> Response {
        self.0.recv().unwrap()
    }
}

/// A type that is shared between the Client and parts that need to control the client
#[derive(Clone)]
pub struct ClientHandle {
    command_sender: Sender<(Command, Sender<Response>)>,
    addr: SocketAddr
}

impl ClientHandle {
    pub fn send_command(&self, command: Command) -> ResponseFuture {
        let (response_sender, response_receiver) = mpsc::channel();
        self.command_sender.send((command, response_sender)).unwrap();
        ResponseFuture(response_receiver)
    }
    
    pub fn addr(&self) -> &SocketAddr {
        &self.addr
    }
}

/// The type that handles clients
pub struct Client {
    framer: LengthDelineated<BufReader<TcpStream>, BufWriter<TcpStream>>,
    command_receiver: Receiver<(Command, Sender<Response>)>,
}

impl Client {
    pub fn new(stream: TcpStream) -> (Self, ClientHandle) {
        let addr = stream.peer_addr().unwrap();
        log::info!("New client {:?}", addr);
        let (command_sender, command_receiver) = mpsc::channel();
        let handle = ClientHandle { command_sender, addr };
        (Self {
            framer: LengthDelineated::new(BufReader::new(stream.try_clone().unwrap()), BufWriter::new(stream)),
            command_receiver,
        }, handle)
    }
    
    pub fn run(&mut self) {
        loop {
            let Ok((command, response_sender)) = self.command_receiver.recv() else {
                log::info!("Client closed");
                break;
            };
            log::debug!("New command {:?}", command);
            
            if let Err(e) = self.framer.send_message(&command) {
                log::error!("Could not send command: {:?}", e);
                break;
            }
            log::debug!("Sent command, waiting for response");
            
            let response = match self.framer.receive_message::<Response>() {
                Ok(Ok(r)) => r,
                Ok(Err(e)) => {
                    log::error!("Could not deserialize response: {:?}", e);
                    break;
                }
                Err(e) => {
                    log::error!("Could not receive response: {:?}", e);
                    break;
                }
            };
            
            if let Err(e) = response_sender.send(response) {
                log::error!("Could not send response: {:?}", e);
                break;
            }
            log::debug!("Sent response");
        }
    }
}

#[derive(Default)]
pub struct ServerHandle {
    pub clients: Mutex<Vec<ClientHandle>>,
}

pub struct Server {
    listener: TcpListener,
    handle: Arc<ServerHandle>,
}

impl Server {
    pub fn new(listener: TcpListener) -> Self {
        Self {
            listener,
            handle: Arc::new(ServerHandle::default()),
        }
    }
    
    pub fn handle(&self) -> Arc<ServerHandle> {
        Arc::clone(&self.handle)
    }
    
    pub fn run(&mut self) {
        log::info!("Server listening on {:?}", self.listener.local_addr().unwrap());
        for stream in self.listener.incoming() {
            let stream = match stream {
                Ok(n) => n,
                Err(e) => {
                    log::error!("Could not get stream: {:?}", e);
                    continue;
                }
            };
            
            let (mut client, handle) = Client::new(stream);
            self.handle.clients.lock().unwrap().push(handle);
            thread::spawn(move || client.run());
        }
    }

}

/// Read a command from stdin by first printing "> "
fn read_command() -> String {
    // Print the prompt
    print!("> ");
    // Flush stdout to ensure the prompt is displayed before reading input
    io::stdout().flush().expect("Failed to flush stdout");

    // Read the command from stdin
    let mut command = String::new();
    io::stdin().read_line(&mut command).expect("Failed to read line");

    // Trim any trailing newline or whitespace and return the command
    command.trim().to_string()
}

fn main() -> io::Result<()> {
    pretty_env_logger::init();
    
    let listener = TcpListener::bind("127.0.0.1:4000")?;
    let mut server = Server::new(listener);
    let handle = server.handle();
    thread::spawn(move || server.run());
    
    loop {
        let command = read_command();
        let args = command.split_whitespace().collect::<Vec<_>>();
        match args.as_slice() {
            ["clients"] => {
                let clients = handle.clients.lock().unwrap();
                for client in clients.iter() {
                    println!("{:?}", client.addr());
                }
            }
            ["send", addr, command @ ..] => {
                let addr = addr.parse().expect("Could not parse address");
                let command = command.join(" ");
                let clients = handle.clients.lock().unwrap();
                let client = clients.iter().find(|c| c.addr() == &addr);
                match client {
                    Some(client) => {
                        let response = client.send_command(Command::RunCommand(command)).wait();
                        match response {
                            Response::RunCommandOutput(output) => println!("{}", output),
                            Response::Success => println!("Success"),
                            Response::Error(code) => println!("Error: {}", code),
                        }
                    }
                    None => println!("Client not found"),
                }
            }
            ["send-all", command @ ..] => {
                let command = command.join(" ");
                let clients = handle.clients.lock().unwrap();
                for client in clients.iter() {
                    let response = client.send_command(Command::RunCommand(command.clone())).wait();
                    match response {
                        Response::RunCommandOutput(output) => println!("{}", output),
                        Response::Success => println!("Success"),
                        Response::Error(code) => println!("Error: {}", code),
                    }
                }
            }
            ["exit"] => break,
            _ => println!("Invalid command"),
        }
    }
    
    Ok(())
}

# Rust Remote Access Trojan (RAT)

This project implements a Remote Access Trojan (RAT) system in Rust, allowing remote command execution and management of multiple clients.

## Disclaimer and Responsible Use

**IMPORTANT:** This project is for educational and research purposes only. The use of this software for any malicious or unauthorized activities is strictly prohibited and illegal. Always obtain explicit permission before running this software on any system you do not own or have authorization to access.

By using this software, you agree to:
1. Only use it on systems you own or have explicit permission to access.
2. Comply with all applicable laws and regulations.
3. Not use it for any malicious, harmful, or illegal activities.

The authors and contributors of this project are not responsible for any misuse or damage caused by this software.

## Project Structure

The project is organized as a Rust workspace with three main crates:

- `client`: Implements the client-side functionality.
- `server`: Implements the server-side functionality.
- `shared`: Contains shared code used by both client and server.

## Features

- TCP-based communication between clients and server.
- Custom serialization and deserialization for commands and responses.
- Length-prefixed framing protocol for message transmission.
- Server can manage multiple client connections.
- Remote command execution on clients.
- Server-side CLI for managing clients and sending commands.

## Getting Started

### Prerequisites

- Rust and Cargo (latest stable version)

### Building the Project

1. Clone the repository:
   ```
   git clone <repository-url>
   cd <project-directory>
   ```

2. Build the project:
   ```
   cargo build --release
   ```

### Running the Server

1. Start the server:
   ```
   cargo run --release --bin server
   ```

2. The server will start listening on `127.0.0.1:4000`.

### Running a Client

1. In a separate terminal, start a client:
   ```
   cargo run --release --bin client
   ```

2. The client will automatically connect to the server at `localhost:4000`.

## Server CLI Commands

- `clients`: List all connected clients.
- `send <client-address> <command>`: Send a command to a specific client.
- `send-all <command>`: Send a command to all connected clients.
- `exit`: Shut down the server.

## Implementation Details

### Shared Crate

- Defines `Serialize` and `Deserialize` traits for message encoding/decoding.
- Implements `Framer` trait for sending and receiving data frames.
- Defines `Command` and `Response` enums for client-server communication.
- Implements `LengthDelineated` protocol for message framing.

### Client

- Connects to the server using a TCP stream.
- Listens for commands from the server and executes them.
- Can execute shell commands and print to console.
- Sends execution results back to the server.

### Server

- Manages multiple client connections using threads.
- Provides a CLI for the server operator to manage clients and send commands.
- Handles responses from clients asynchronously.

## Future Improvements

- Implement authentication and encryption for secure communication.
- Add more sophisticated command types and responses.
- Implement error handling and recovery mechanisms.
- Add unit and integration tests.
- Enhance logging and monitoring capabilities.
- Implement additional security measures to prevent misuse.

## Legal and Ethical Considerations

Before using or modifying this software, ensure you understand and comply with all relevant laws and regulations in your jurisdiction. Unauthorized access to computer systems is illegal and unethical. Always prioritize privacy, security, and obtain proper authorization for any penetration testing or security research activities.

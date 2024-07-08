use crate::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub enum Command {
    RunCommand(String),
    ConsolePrint(String)
}

#[derive(Clone, Debug)]
pub enum Response {
    RunCommandOutput(String),
    Success,
    Error(i32),
}

impl Serialize for Command {
    fn to_bytes(&self) -> Vec<u8> {
        let (id, buf) = match self {
            Command::RunCommand(command) => (0u8, command.as_bytes()),
            Command::ConsolePrint(text) => (1, text.as_bytes()),
        };
        
        let mut bytes = vec![id];
        bytes.extend_from_slice(buf);
        
        bytes
    }
}

impl Deserialize for Command {
    fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        let id = bytes[0];
        let buf = &bytes[1..];
        
        match id {
            0 => String::from_utf8(buf.to_vec()).map_err(|e| e.to_string()).map(Command::RunCommand),
            1 => String::from_utf8(buf.to_vec()).map_err(|e| e.to_string()).map(Command::ConsolePrint),
            n => Err(format!("Invalid command {}", id)),
        }
    }
}

impl Serialize for Response {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            Response::RunCommandOutput(output) => {
                let mut bytes = vec![0u8];
                bytes.extend_from_slice(output.as_bytes());
                bytes
            }
            Response::Success => vec![1u8],
            Response::Error(code) => {
                let mut bytes = vec![2u8];
                bytes.extend_from_slice(&code.to_le_bytes());
                bytes
            }
        }
    }
}

impl Deserialize for Response {
    fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        let id = bytes[0];
        let buf = &bytes[1..];

        match id {
            0 => String::from_utf8(buf.to_vec())
                .map_err(|e| e.to_string())
                .map(Response::RunCommandOutput),
            1 => Ok(Response::Success),
            2 => {
                if buf.len() != 4 {
                    return Err("Invalid length for Error code".to_string());
                }
                let code = i32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
                Ok(Response::Error(code))
            }
            n => Err(format!("Invalid response {}", id)),
        }
    }
}


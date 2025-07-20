use crate::{MpvCommand, Response};
use std::io::prelude::*;
use std::net::Shutdown;
use std::os::unix::net::UnixStream;

pub fn send_command(socket_path: &str, command: MpvCommand) -> Result<Response, std::io::Error> {
    // Sends a raw json command to the specified mpv socket

    let json_command: String = serde_json::to_string(&command)?;
    println!("{}", json_command);

    // Check if there is a newline at the end
    let command = if json_command.ends_with('\n') {
        json_command
    } else {
        format!("{json_command}\n")
    };

    // Connect to MPV socket
    let mut stream = UnixStream::connect(socket_path)?;

    // Send the command
    stream.write_all(command.as_bytes())?;
    stream.flush()?;

    // Get response
    let mut response: [u8; 1024] = [0; 1024];
    stream.read(&mut response)?;

    // Filter the response by removing all the \0
    let mut filtered_response = String::new();
    for i in response {
        let to_char = i as char;
        if to_char != '\0' {
            filtered_response.push(to_char);
        }
    }

    stream.shutdown(Shutdown::Both)?;
    Ok(serde_json::from_str(&filtered_response)?)
}

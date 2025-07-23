use crate::config::BUFFER_SIZE;
use crate::util::{filter_buffer, mpv_command_to_json, parse_response_string};
use crate::{MpvCommand, Response};
use std::io::prelude::*;
use std::os::unix::net::UnixStream;

pub fn send_command(
    mut stream: &UnixStream,
    command: MpvCommand,
) -> Result<[u8; 4096], std::io::Error> {
    // Sends a raw json command to the specified mpv socket
    // This might return multiple responses including events

    let json_command = mpv_command_to_json(command)?;

    // Send the command
    stream.write_all(json_command.as_bytes())?;
    stream.flush()?;

    // Get response
    let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    stream.read(&mut buffer)?;

    return Ok(buffer);
}

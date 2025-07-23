use serde_json::Error;

use crate::{MpvCommand, Response, config::BUFFER_SIZE};

pub fn mpv_command_to_json(command: MpvCommand) -> Result<String, Error> {
    let json_command: String = serde_json::to_string(&command)?;

    // Check if there is a newline at the end
    if json_command.ends_with('\n') {
        return Ok(json_command);
    } else {
        return Ok(format!("{json_command}\n"));
    };
}

pub fn filter_buffer(buffer: [u8; BUFFER_SIZE]) -> String {
    let mut filtered_buffer = String::new();

    for i in buffer {
        let to_char = i as char;
        if to_char != '\0' {
            filtered_buffer.push(to_char);
        }
    }

    return filtered_buffer;
}

pub fn parse_response_string(all_responses: String) -> Vec<Response> {
    let split_responses = all_responses.split_whitespace();
    let mut responses: Vec<Response> = Vec::new();

    for i in split_responses {
        let parsed: Response = match serde_json::from_str(i) {
            Ok(resp) => resp,
            Err(e) => panic!("Problem with parsing this response from the MPV IPC: {i}\n{e}"),
        };

        responses.push(parsed);
    }

    return responses;
}


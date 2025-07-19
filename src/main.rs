use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::net::Shutdown;
use std::os::unix::net::UnixStream;

// fn to_chars(array, array_size: u32) -> String {
//     // this converts an array to a string
//     return String::new()
// }
#[derive(Debug, Serialize, Deserialize)]
struct Response {
    data: Option<bool>,
    request_id: usize,
    error: String,
}

// #[derive(Debug, Serialize, Deserialize)]
// struct Command {
//     command
// }

fn main() -> std::io::Result<()> {
    let json_str = r#"{"command":["set_property","pause", true]}"#;
    let command = format!("{}\n", json_str); // Append END\n

    // Connect to MPV socket
    let mut stream = UnixStream::connect("/tmp/mpv-socket")?;

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

    let deserialized_response: Response = serde_json::from_str(&filtered_response)?;
    println!("{:?}", deserialized_response);

    stream.shutdown(Shutdown::Both)?;
    Ok(())
}

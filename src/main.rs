use crate::config::MPV_SOCKET_PATH;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::Value;
mod config;
mod mpv_ipc;

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    data: Option<Value>,
    request_id: usize,
    error: String,
}

#[derive(Debug, Serialize)]
struct MpvCommand {
    command: CommandType,
    request_id: i64,
    #[serde(rename = "async")]
    is_async: bool,
}

// Thanks to the docs: https://github.com/mpv-player/mpv/blob/master/DOCS/man/ipc.rst#id7
#[derive(Debug)]
enum CommandType {
    SetProperty { property: Property, value: Value },
    GetProperty { property: Property },
    ObserveProperty { property: Property, id: i64 },
    GetVersion {},
    ClientName {},
    GetTimeUs {},
}

// Thanks to MPV man page for this
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum Property {
    AudioSpeedCorrection,
    VideoSpeedCorrection,
    DisplaySyncActive,
    Filename,
    FileSize,
    EstimatedFrameCount,
    EstimatedFrameNumber,
    Pid,
    Path,
    StreamOpenFilename,
    MediaTitle,
    FileFormat,
    CurrentDemuxer,
    StreamPath,
    StreamPos,
    StreamEnd,
    Duration,
}

impl Serialize for CommandType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde_json::json;

        let array = match self {
            CommandType::SetProperty { property, value } => {
                json!(["set_property", property, value])
            }
            CommandType::GetProperty { property } => {
                json!(["get_property", property])
            }
            CommandType::GetVersion {} => {
                json!(["get_version"])
            }
            CommandType::ClientName {} => {
                json!(["client_name"])
            }
            CommandType::GetTimeUs {} => {
                json!(["get_time_us"])
            }
            CommandType::ObserveProperty { id, property } => {
                json!(["observe_property", id, property])
            }
        };

        array.serialize(serializer)
    }
}

fn main() -> std::io::Result<()> {
    let response = mpv_ipc::send_command(
        MPV_SOCKET_PATH,
        MpvCommand {
            request_id: 0,
            command: CommandType::GetProperty {
                property: Property::Duration,
            },
            is_async: false,
        },
    )?;

    println!("{:?}", response);

    Ok(())
}

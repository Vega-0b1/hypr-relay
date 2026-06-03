use crate::notification;
use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::net::UnixStream;

pub fn run(socket_path: &dyn Fn(&str) -> String) {
    let stream = UnixStream::connect(socket_path(".socket2.sock"))
        .expect("could not connect to Hyprland event socket");

    for line in BufReader::new(stream).lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };

        if line.starts_with("workspace>>") {
            let (id, name) = active_workspace(socket_path);
            notification::send("workspace", 9992, 1000, &format!("Workspace {id}"), &name);
        }
    }
}

fn active_workspace(socket_path: &dyn Fn(&str) -> String) -> (i32, String) {
    let Ok(mut stream) = UnixStream::connect(socket_path(".socket.sock")) else {
        return (0, "?".to_string());
    };
    stream.write_all(b"j/activeworkspace").ok();

    let mut response = Vec::new();
    let mut buf = [0u8; 8192];
    loop {
        let n = stream.read(&mut buf).unwrap_or(0);
        response.extend_from_slice(&buf[..n]);
        if n == 0 || n < 8192 {
            break;
        }
    }

    let response = String::from_utf8_lossy(&response);
    let json: serde_json::Value =
        serde_json::from_str(&response).unwrap_or(serde_json::Value::Null);

    let id = json["id"].as_i64().unwrap_or(0) as i32;
    let name = json["name"].as_str().unwrap_or("?").to_string();

    (id, name)
}

use crate::notification;
use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::net::UnixStream;

pub fn daemon(socket_path: &dyn Fn(&str) -> String) {
    // reconnect if the Hyprland event socket drops
    loop {
        match UnixStream::connect(socket_path(".socket2.sock")) {
            Ok(stream) => {
                for line in BufReader::new(stream).lines() {
                    let line = match line {
                        Ok(l) => l,
                        Err(_) => break,
                    };

                    if line.starts_with("workspace>>") {
                        let (id, name) = active_workspace(socket_path);
                        notification::send(
                            "workspace",
                            9992,
                            1000,
                            &format!("Workspace {id}"),
                            &name,
                        );
                    }
                }
            }
            Err(e) => eprintln!("hypr-relay: could not connect to Hyprland event socket: {e}"),
        }
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}

fn active_workspace(socket_path: &dyn Fn(&str) -> String) -> (i32, String) {
    let Ok(mut stream) = UnixStream::connect(socket_path(".socket.sock")) else {
        return (0, "?".to_string());
    };
    stream.write_all(b"j/activeworkspace").ok();

    // Hyprland closes the request socket after replying, so read to EOF
    let mut response = Vec::new();
    stream.read_to_end(&mut response).ok();

    let response = String::from_utf8_lossy(&response);
    let json: serde_json::Value =
        serde_json::from_str(&response).unwrap_or(serde_json::Value::Null);

    let id = json["id"].as_i64().unwrap_or(0) as i32;
    let name = json["name"].as_str().unwrap_or("?").to_string();

    (id, name)
}

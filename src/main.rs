mod bluetooth;
mod brightness;
mod notification;
mod volume;
mod workspace;

use std::env;

fn socket_path(socket: &str) -> String {
    let sig = env::var("HYPRLAND_INSTANCE_SIGNATURE").expect("HYPRLAND_INSTANCE_SIGNATURE not set");

    let uid = unsafe { libc::getuid() };
    let runtime = env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| format!("/run/user/{uid}"));

    let xdg_path = format!("{runtime}/hypr/{sig}/{socket}");
    let tmp_path = format!("/tmp/hypr/{sig}/{socket}");

    if std::path::Path::new(&xdg_path).exists() {
        xdg_path
    } else {
        tmp_path
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        None => daemon(),
        Some("--version" | "-V") => println!("hypr-relay {}", env!("CARGO_PKG_VERSION")),
        Some("--help" | "-h") => println!("usage: hypr-relay"),
        _ => {
            eprintln!("usage: hypr-relay");
            std::process::exit(1);
        }
    }
}

fn daemon() {
    let mut handles = Vec::new();

    if env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok() {
        handles.push(std::thread::spawn(|| workspace::daemon(&socket_path)));
    } else {
        eprintln!("hypr-relay: HYPRLAND_INSTANCE_SIGNATURE not set - workspace module disabled");
    }

    handles.push(std::thread::spawn(bluetooth::daemon));
    handles.push(std::thread::spawn(volume::daemon));
    handles.push(std::thread::spawn(brightness::daemon));

    for h in handles {
        let _ = h.join();
    }
}

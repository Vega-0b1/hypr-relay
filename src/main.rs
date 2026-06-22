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
        _ => eprintln!("usage: hypr-relay"),
    }
}

fn daemon() {
    let workspace = std::thread::spawn(|| workspace::daemon(&socket_path));
    let bluetooth = std::thread::spawn(|| bluetooth::daemon());
    let volume = std::thread::spawn(|| volume::daemon());
    let brightness = std::thread::spawn(|| brightness::daemon());

    workspace.join().unwrap();
    bluetooth.join().unwrap();
    volume.join().unwrap();
    brightness.join().unwrap();
}

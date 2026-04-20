use crate::notification;
use std::process::Command;

pub fn run(args: &[String]) {
    let action = args.get(0).map(|arg| arg.as_str()).unwrap_or("");
    let step = args.get(1).map(|arg| arg.as_str()).unwrap_or("5");
    let sink = "@DEFAULT_AUDIO_SINK@";

    match action {
        "up" | "down" => {
            let step_val: u32 = step.parse().unwrap_or(5);
            let current = Command::new("wpctl")
                .args(["get-volume", sink])
                .output()
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .and_then(|s| {
                    s.split_whitespace()
                        .nth(1)
                        .and_then(|v| v.parse::<f32>().ok())
                })
                .map(|v| (v * 100.0) as u32)
                .unwrap_or(0);
            let target = if action == "up" {
                ((current / step_val) + 1) * step_val
            } else {
                current.saturating_sub(1) / step_val * step_val
            };
            Command::new("wpctl")
                .args(["set-mute", sink, "0"])
                .status()
                .ok();
            Command::new("wpctl")
                .args(["set-volume", sink, &format!("{target}%"), "--limit", "1.0"])
                .status()
                .ok();
        }
        "mute" => {
            Command::new("wpctl")
                .args(["set-mute", sink, "toggle"])
                .status()
                .ok();
        }
        _ => return,
    }

    let Ok(out) = Command::new("wpctl").args(["get-volume", sink]).output() else {
        return;
    };
    let out = String::from_utf8_lossy(&out.stdout);

    let muted = out.contains("[MUTED]");

    let vol: f32 = out
        .split_whitespace()
        .nth(1)
        .and_then(|arg| arg.parse().ok())
        .unwrap_or(0.0);

    let percentage = (vol * 100.0).round() as u32;

    if muted {
        notification::send("volume", 9990, 1000, "Volume Muted", "");
    } else {
        notification::send("volume", 9990, 1000, &format!("Volume {percentage}%"), "");
    }
}

pub fn mic_toggle() {
    let source = "@DEFAULT_AUDIO_SOURCE@";
    Command::new("wpctl")
        .args(["set-mute", source, "toggle"])
        .status()
        .ok();

    let Ok(out) = Command::new("wpctl").args(["get-volume", source]).output() else {
        return;
    };
    let out = String::from_utf8_lossy(&out.stdout);
    let muted = out.contains("[MUTED]");

    if muted {
        notification::send("microphone", 9994, 1000, "Mic Muted", "");
    } else {
        notification::send("microphone", 9994, 1000, "Mic Active", "");
    }
}

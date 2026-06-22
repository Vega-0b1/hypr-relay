use crate::notification;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

pub fn daemon() {
    let Ok(mut child) = Command::new("pactl")
        .arg("subscribe")
        .stdout(Stdio::piped())
        .spawn()
    else {
        return;
    };

    let stdout = child.stdout.take().unwrap();
    let start = std::time::Instant::now();

    for line in BufReader::new(stdout).lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };

        if start.elapsed().as_secs() < 2 {
            continue;
        }

        if !line.contains(" on sink #") {
            continue;
        }

        let Ok(out) = Command::new("wpctl")
            .args(["get-volume", "@DEFAULT_AUDIO_SINK@"])
            .output()
        else {
            continue;
        };

        let out = String::from_utf8_lossy(&out.stdout);
        let muted = out.contains("[MUTED]");
        let vol: f32 = out
            .split_whitespace()
            .nth(1)
            .and_then(|v| v.parse().ok())
            .unwrap_or(0.0);

        let percentage = (vol * 100.0).round() as u32;

        if muted {
            notification::send("volume", 9990, 1000, "Volume Muted", "");
        } else {
            notification::send("volume", 9990, 1000, &format!("Volume {percentage}%"), "");
        }
    }
}

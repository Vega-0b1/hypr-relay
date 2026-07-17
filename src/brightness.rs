use crate::notification;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

pub fn daemon() {
    // respawn the listener if udevadm dies
    loop {
        if !run() {
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}

// returns false if udevadm couldn't be spawned at all (feature disabled)
fn run() -> bool {
    let Ok(mut child) = Command::new("udevadm")
        .args(["monitor", "--kernel", "--subsystem-match=backlight"])
        .stdout(Stdio::piped())
        .spawn()
    else {
        return false;
    };

    let stdout = child.stdout.take().unwrap();

    for line in BufReader::new(stdout).lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        if !line.contains("change") {
            continue;
        }

        let Ok(current) = Command::new("brightnessctl").args(["get"]).output() else {
            continue;
        };

        let Ok(max) = Command::new("brightnessctl").args(["max"]).output() else {
            continue;
        };

        let current: u32 = String::from_utf8_lossy(&current.stdout)
            .trim()
            .parse()
            .unwrap_or(0);
        let max: u32 = String::from_utf8_lossy(&max.stdout)
            .trim()
            .parse()
            .unwrap_or(1)
            .max(1);

        let percentage = current * 100 / max;

        notification::send(
            "brightness",
            9991,
            1000,
            &format!("Brightness {percentage}%"),
            "",
        );
    }

    let _ = child.kill();
    let _ = child.wait();
    true
}

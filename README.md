# hypr-relay

A lightweight daemon for Hyprland that bridges system events to your notification daemon.

Runs as a single background process and automatically sends D-Bus notifications for workspace switches, Bluetooth connections, volume changes, and brightness changes — no keybind configuration required.

## Features

- Workspace change notifications (via Hyprland IPC)
- Bluetooth device connect/disconnect notifications
- Volume change notifications (via PipeWire)
- Brightness change notifications (via udev)

## Dependencies

- [Hyprland](https://hyprland.org)
- A Freedesktop-compatible notification daemon (`dunst`, `mako`, `swaync`, etc.)
- `pactl` - volume event monitoring (PipeWire/PulseAudio)
- `wpctl` - volume state querying (PipeWire)
- `brightnessctl` - brightness state querying
- `bluetoothctl` - Bluetooth events

## Installation

```bash
git clone https://github.com/Vega-0b1/hypr-relay
cd hypr-relay
cargo build --release
cp target/release/hypr-relay ~/.local/bin/
```

## Usage

Start the daemon:

```
hypr-relay
```

Add to your Hyprland config to start on login:

```
exec-once = hypr-relay
```

Since hypr-relay listens for system events directly, your keybinds call the underlying tools as normal:

```
bindel = , XF86AudioRaiseVolume,  exec, wpctl set-volume @DEFAULT_AUDIO_SINK@ 5%+
bindel = , XF86AudioLowerVolume,  exec, wpctl set-volume @DEFAULT_AUDIO_SINK@ 5%-
bindel = , XF86AudioMute,         exec, wpctl set-mute @DEFAULT_AUDIO_SINK@ toggle
bindel = , XF86AudioMicMute,      exec, wpctl set-mute @DEFAULT_AUDIO_SOURCE@ toggle
bindel = , XF86MonBrightnessUp,   exec, brightnessctl set 5%+
bindel = , XF86MonBrightnessDown, exec, brightnessctl set 5%-
```

## License

MIT

 # hypr-relay

  A lightweight daemon for Hyprland that bridges system events to your notification daemon.

  Listens for workspace switches and Bluetooth connections via Hyprland IPC, and exposes CLI commands for volume and
  brightness - all forwarded as standard D-Bus notifications.

  ## Features

  - Workspace change notifications (via Hyprland socket)
  - Bluetooth device connect/disconnect notifications
  - Volume control with mute support
  - Brightness control

  ## Dependencies

  - [Hyprland](https://hyprland.org)
  - A Freedesktop-compatible notification daemon (`dunst`, `mako`, `swaync`, etc.)
  - `wpctl` - volume control (PipeWire)
  - `brightnessctl` - brightness control
  - `bluetoothctl` - Bluetooth events

  ## Installation

  ```bash
  git clone https://github.com/Vega-0b1/hypr-relay
  cd hypr-relay
  cargo build --release
  cp target/release/hypr-relay ~/.local/bin/

  Usage

  Daemon

  Start the daemon to enable workspace and Bluetooth notifications:

  hypr-relay

  Add to your Hyprland config to start on login:

  exec-once = hypr-relay

  Volume

  hypr-relay volume up [step]
  hypr-relay volume down [step]
  hypr-relay volume mute

  Example keybinds in hyprland.conf:

  bindel = , XF86AudioRaiseVolume, exec, hypr-relay volume up 5
  bindel = , XF86AudioLowerVolume, exec, hypr-relay volume down 5
  bindel = , XF86AudioMute,        exec, hypr-relay volume mute

  Brightness

  hypr-relay brightness up [step]
  hypr-relay brightness down [step]

  Example keybinds:

  bindel = , XF86MonBrightnessUp,   exec, hypr-relay brightness up 5
  bindel = , XF86MonBrightnessDown, exec, hypr-relay brightness down 5

  License

  MIT
  ```

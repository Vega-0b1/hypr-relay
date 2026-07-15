# hypr-relay

A lightweight daemon for Hyprland that bridges system events to your notification daemon.

Runs as a single background process and sends D-Bus notifications for volume, brightness, workspace, and Bluetooth changes. No keybind configuration required.

|                                |                                  |
| :----------------------------: | :------------------------------: |
|  ![Volume](assets/volume.png)  |      ![Muted](assets/mute.png)   |
| ![Workspace](assets/workspace.png) | ![Bluetooth](assets/bluetooth.png) |

## How it works

hypr-relay doesn't handle keybinds. Instead it listens for the events themselves:
Hyprland's IPC socket, PipeWire sink changes, backlight udev events, and BlueZ device
events. Your keybinds keep calling `wpctl`/`brightnessctl` as normal, and notifications
appear no matter what triggered the change (keybind, hardware key, GUI, another device).

Each event source runs on its own listener thread inside one process. Notifications use
fixed IDs and the `x-canonical-private-synchronous` hint, so rapid changes (like holding
a volume key) update a single notification in place instead of stacking.

## Features

| Feature    | Event source                          | Requires                    |
| ---------- | ------------------------------------- | --------------------------- |
| Volume     | `pactl subscribe`                     | `pactl`, `wpctl` (PipeWire) |
| Brightness | `udevadm monitor` (backlight)         | `brightnessctl`             |
| Workspace  | Hyprland IPC                          | nothing                     |
| Bluetooth  | `bluetoothctl` event stream           | `bluetoothctl` (BlueZ)      |

If a tool is missing, that feature is simply disabled. The rest keep working.

You'll also need a Freedesktop-compatible notification daemon (`mako`, `dunst`,
`swaync`, etc.).

## Installation

### Arch Linux ([AUR](https://aur.archlinux.org/packages/hypr-relay))

```bash
yay -S hypr-relay
```

Or build the pacman package manually from the included PKGBUILD:

```bash
git clone https://github.com/Vega-0b1/hypr-relay
cd hypr-relay
makepkg -si
```

### NixOS (flake input)

Add the repo as a (non-flake) input and expose it through an overlay:

```nix
# flake.nix
inputs.hypr-relay = {
  url = "github:Vega-0b1/hypr-relay";
  flake = false;
};
```

```nix
# in your NixOS configuration
nixpkgs.overlays = [
  (final: prev: { hypr-relay = prev.callPackage hypr-relay {}; })
];
environment.systemPackages = [ pkgs.hypr-relay ];
```

### From source

```bash
git clone https://github.com/Vega-0b1/hypr-relay
cd hypr-relay
cargo build --release
cp target/release/hypr-relay ~/.local/bin/
```

## Usage

Add one line to your Hyprland config:

```
exec-once = hypr-relay
```

That's the entire setup. With a default Hyprland config everything works out of the
box, and your existing keybinds keep working as-is:

```
bindel = , XF86AudioRaiseVolume,  exec, wpctl set-volume @DEFAULT_AUDIO_SINK@ 5%+
bindel = , XF86AudioLowerVolume,  exec, wpctl set-volume @DEFAULT_AUDIO_SINK@ 5%-
bindel = , XF86AudioMute,         exec, wpctl set-mute @DEFAULT_AUDIO_SINK@ toggle
bindel = , XF86AudioMicMute,      exec, wpctl set-mute @DEFAULT_AUDIO_SOURCE@ toggle
bindel = , XF86MonBrightnessUp,   exec, brightnessctl set 5%+
bindel = , XF86MonBrightnessDown, exec, brightnessctl set 5%-
```

### Workspace names

The second line of a workspace notification is the workspace's name. By default that
is just the workspace number. To show something more useful, give your workspaces
names with [workspace rules](https://wiki.hypr.land/Configuring/Basics/Workspace-Rules/)
in `hyprland.conf`:

```
workspace = 1, defaultName:Main
workspace = 2, defaultName:Code
```

With these rules, switching to workspace 2 shows "Workspace 2" with "Code" below it,
as in the screenshot at the top.

## License

MIT

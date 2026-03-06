# RMF Site Editor (Desktop Fork)

A desktop-focused fork of [open-rmf/rmf_site](https://github.com/open-rmf/rmf_site), the visual editor for designing and managing robot fleet management (RMF) sites.

Built in Rust with [Bevy](https://bevyengine.org/) and [egui](https://github.com/emilk/egui). Targets Linux as the primary platform, with .deb and AppImage packaging.

## What this fork changes

Improvements over the upstream project:

- Save/export feedback: window title shows unsaved state, toast notifications on save/export, exit confirmation dialog
- Edit menu with Undo/Redo/Delete and keyboard shortcuts
- File menu with New/Open entries
- Snap-to-grid with configurable presets (`G` to toggle, `Shift+G` to cycle grid size)
- Status bar showing cursor world coordinates and snap state
- Light export in SDF (point, spot, directional)
- ROS 2 launch file auto-generation alongside SDF export
- Desktop packaging: .deb (cargo-deb) and AppImage (linuxdeploy)
- CI release workflow for automated builds on tagged versions

## Download

Pre-built packages are available on the [Releases](https://github.com/MaximilianCF/rmf_site/releases) page:

- `.deb` for Debian/Ubuntu-based distros
- `.AppImage` portable binary for any Linux distro

## Building from source

### Dependencies (Ubuntu/Debian)

```
sudo apt install libgtk-3-dev libasound2-dev libudev-dev
```

Install Rust via [rustup](https://www.rust-lang.org/tools/install) if you don't have it:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Run

```
cargo run
```

Use `--features bevy/dynamic_linking` for faster compile times during development.
Use `--release` for better runtime performance.

### Build .deb package

```
cargo install cargo-deb
cargo deb -p rmf_site_editor
```

Output goes to `target/debian/`.

### Build AppImage

```
cargo build --release --bin rmf_site_editor
bash packaging/build-appimage.sh
```

Requires [linuxdeploy](https://github.com/linuxdeploy/linuxdeploy). The script will download it automatically if not found.

## Keyboard shortcuts

| Key | Action |
|-----|--------|
| Ctrl+N | New workspace |
| Ctrl+O | Open file |
| Ctrl+S | Save |
| Ctrl+Shift+S | Save As |
| Ctrl+E | Export SDF |
| Ctrl+Z | Undo |
| Ctrl+Shift+Z | Redo |
| Delete | Delete selected |
| G | Toggle snap-to-grid |
| Shift+G | Cycle grid size |

## Project structure

Rust workspace with crates under `crates/`:

- `rmf_site_editor` -- main application binary and editor logic
- `rmf_site_format` -- site data model, serialization, SDF export
- `rmf_site_egui` -- egui-based menu bar and UI widgets
- `rmf_site_camera` -- 3D camera controls
- `rmf_site_picking` -- mouse picking / selection
- `rmf_site_mesh` -- mesh generation for walls, floors, doors
- `rmf_site_animate` -- animation utilities

## ROS 2 integration

To use exported sites in an Open-RMF project, see [`rmf_site_ros2`](https://github.com/open-rmf/rmf_site_ros2). The SDF export now includes a `launch.py` file that can be used directly with `ros2 launch`.

## License

Apache-2.0. See [LICENSE](LICENSE).

## Upstream

This is a fork of [open-rmf/rmf_site](https://github.com/open-rmf/rmf_site). Original work by Open Source Robotics Foundation contributors.

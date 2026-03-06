# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

The RMF Site Editor is a visual editor for large RMF (Robot Middleware Framework) deployment sites. Built in Rust using the **Bevy 0.16** game engine, it targets both desktop (Linux/Windows/Mac) and web (WebAssembly+WebGL/WebGPU).

## Build & Run Commands

### Desktop
```bash
cargo run                                    # Build and run
cargo run --features bevy/dynamic_linking    # Faster incremental compile via dynamic linking
cargo run --release                          # Release build (better runtime performance)
```

### WebAssembly
```bash
scripts/build-web.sh    # Build WASM binary (requires wasm-bindgen-cli 0.2.100, wasm-opt)
scripts/serve-web.sh    # Serve at http://localhost:1234
```

### Packaging (Desktop)
```bash
cargo install cargo-deb                                     # Install cargo-deb (one-time)
cargo deb -p rmf_site_editor                                # Build .deb package
bash packaging/build-appimage.sh                            # Build AppImage (needs linuxdeploy)
```

Push a `v*` tag to trigger the `release.yaml` CI workflow which builds both .deb and AppImage and creates a GitHub Release.

### Testing
```bash
cargo test -p rmf_site_format     # Test the format crate
cargo test -p rmf_site_editor     # Test the editor crate
cargo test -p <crate_name>        # Test a specific crate
```

### Linting & Formatting
```bash
cargo fmt --check    # Check formatting (CI enforced)
cargo fmt            # Fix formatting
cd crates/rmf_site_format && cargo check --no-default-features  # Minimal feature build (CI enforced)
```

Pre-commit hooks run `cargo fmt` and `cargo check` automatically.

## Workspace Architecture

Cargo workspace with 8 crates under `crates/`:

| Crate | Role |
|---|---|
| `rmf_site_format` | **Core data model.** Serialization-focused site/building format definitions (`.site.json`, `.building.yaml`). Optional `bevy` feature gates Bevy component derives. No Bevy runtime dependency by default. |
| `rmf_site_editor` | **Main application.** Bevy app with ECS plugins for site editing, 3D rendering, interaction, undo/redo, SDF export, and nav graph export. Contains the `SiteEditor` plugin and `AppState` state machine (`MainMenu`, `SiteEditor`, `SiteVisualizer`, `SiteDrawingEditor`). |
| `rmf_site_editor_web` | **WASM entry point.** Thin wrapper around `rmf_site_editor` compiled as `cdylib` for browser deployment. |
| `rmf_site_egui` | egui-based UI widgets and panels. |
| `rmf_site_camera` | Camera control systems (orbit, pan, zoom, cursor). |
| `rmf_site_picking` | Entity picking/selection via raycasting. |
| `rmf_site_mesh` | Mesh generation utilities. |
| `rmf_site_animate` | Visual cue animations. |

### Key architectural patterns

- **Bevy ECS plugin architecture**: Each major feature is a Bevy `Plugin` registered in `SiteEditor::build()` (`crates/rmf_site_editor/src/lib.rs`). New features should follow this pattern.
- **Format/editor separation**: `rmf_site_format` is the pure data layer (serde structs), while `rmf_site_editor` adds Bevy ECS components and systems. The `#[cfg(feature = "bevy")]` gate in format types allows them to derive Bevy traits only when used by the editor.
- **Headless mode**: The editor supports `--export_sdf` and `--export_nav` CLI flags for headless batch export without a window, used in CI.
- **Desktop/WASM conditional compilation**: `#[cfg(target_arch = "wasm32")]` and `#[cfg(not(...))]` blocks throughout for platform differences (e.g., clap CLI parsing is desktop-only, async runtime differs).

### Important directories in `rmf_site_editor/src/`

- `site/` — Core site editing logic: loading, saving, SDF export, entity management (levels, lanes, walls, doors, floors, lifts, models, etc.)
- `interaction/` — Entity selection, gizmos, highlights, cursor handling, 3D interaction
- `widgets/` — egui inspector panels and UI widgets
- `workspace/` — Workspace/project management
- `undo/` — Undo/redo system

## Conventions

- Rust stable toolchain (pinned in `crates/rmf_site_editor/rust-toolchain.toml`)
- Workspace dependency versions are centralized in the root `Cargo.toml` `[workspace.dependencies]`
- Dev profile uses `opt-level = 1` for own code and `opt-level = 3` for dependencies (Bevy compile-time optimization)
- The `bevy_egui` version must match the version used by `bevy-inspector-egui`; the `glam` version must match `bevy_math`'s `glam`

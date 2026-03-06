use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{interaction::SnapToGrid, AppState};

use super::RenderUiSet;

/// Tracks the cursor's world-space position for display in the status bar.
#[derive(Resource, Default)]
pub struct CursorWorldPosition {
    pub position: Option<Vec3>,
}

fn update_cursor_world_position(
    mut cursor_pos: ResMut<CursorWorldPosition>,
    intersect: crate::interaction::IntersectGroundPlaneParams,
    cursor_moved: EventReader<CursorMoved>,
) {
    if cursor_moved.is_empty() {
        return;
    }
    cursor_pos.position = intersect
        .ground_plane_intersection()
        .map(|tf| tf.translation);
}

fn render_status_bar(
    mut contexts: EguiContexts,
    cursor_pos: Res<CursorWorldPosition>,
    snap: Res<SnapToGrid>,
) {
    egui::TopBottomPanel::bottom("status_bar")
        .exact_height(22.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.horizontal_centered(|ui| {
                // Cursor coordinates
                if let Some(pos) = cursor_pos.position {
                    ui.label(
                        egui::RichText::new(format!("X: {:.2}  Y: {:.2}", pos.x, pos.y))
                            .monospace()
                            .small(),
                    );
                } else {
                    ui.label(egui::RichText::new("X: ---  Y: ---").monospace().small());
                }

                ui.separator();

                // Snap indicator
                let snap_text = if snap.enabled {
                    format!("Snap: {}m", snap.grid_size)
                } else {
                    "Snap: OFF".to_string()
                };
                let snap_color = if snap.enabled {
                    egui::Color32::from_rgb(100, 200, 120)
                } else {
                    egui::Color32::from_rgb(160, 160, 160)
                };
                ui.label(egui::RichText::new(snap_text).small().color(snap_color));

                ui.separator();

                ui.label(
                    egui::RichText::new("[G] toggle snap  [Shift+G] cycle grid")
                        .small()
                        .weak(),
                );
            });
        });
}

pub struct StatusBarPlugin;

impl Plugin for StatusBarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorWorldPosition>()
            .add_systems(
                Update,
                update_cursor_world_position.run_if(AppState::in_displaying_mode()),
            )
            .add_systems(
                Update,
                render_status_bar
                    .in_set(RenderUiSet)
                    .run_if(AppState::in_displaying_mode()),
            );
    }
}

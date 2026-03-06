use crate::{
    site::Delete,
    undo::{RedoRequest, UndoHistory, UndoRequest},
    AppState,
};
use bevy::{ecs::hierarchy::ChildOf, prelude::*};
use rmf_site_egui::*;
use rmf_site_picking::Selection;

#[derive(Resource)]
pub struct EditMenuItems {
    undo: Entity,
    redo: Entity,
    _separator: Entity,
    delete: Entity,
}

impl FromWorld for EditMenuItems {
    fn from_world(world: &mut World) -> Self {
        let edit_menu = world.resource::<EditMenu>().get();

        let undo = world
            .spawn((
                MenuItem::Text(TextMenuItem::new("Undo").shortcut("Ctrl-Z")),
                ChildOf(edit_menu),
                MenuDisabled,
            ))
            .id();
        let redo = world
            .spawn((
                MenuItem::Text(TextMenuItem::new("Redo").shortcut("Ctrl-Shift-Z")),
                ChildOf(edit_menu),
                MenuDisabled,
            ))
            .id();
        let separator = world.spawn((MenuItem::Separator, ChildOf(edit_menu))).id();
        let delete = world
            .spawn((
                MenuItem::Text(TextMenuItem::new("Delete").shortcut("Del")),
                ChildOf(edit_menu),
                MenuDisabled,
            ))
            .id();

        Self {
            undo,
            redo,
            _separator: separator,
            delete,
        }
    }
}

fn update_edit_menu_state(
    edit_menu: Res<EditMenuItems>,
    undo_history: Res<UndoHistory>,
    selection: Res<Selection>,
    mut commands: Commands,
) {
    if undo_history.can_undo() {
        commands.entity(edit_menu.undo).remove::<MenuDisabled>();
    } else {
        commands.entity(edit_menu.undo).insert(MenuDisabled);
    }

    if undo_history.can_redo() {
        commands.entity(edit_menu.redo).remove::<MenuDisabled>();
    } else {
        commands.entity(edit_menu.redo).insert(MenuDisabled);
    }

    if selection.0.is_some() {
        commands.entity(edit_menu.delete).remove::<MenuDisabled>();
    } else {
        commands.entity(edit_menu.delete).insert(MenuDisabled);
    }
}

fn handle_edit_menu_events(
    mut menu_events: EventReader<MenuEvent>,
    edit_menu: Res<EditMenuItems>,
    selection: Res<Selection>,
    mut undo_request: EventWriter<UndoRequest>,
    mut redo_request: EventWriter<RedoRequest>,
    mut delete: EventWriter<Delete>,
) {
    for event in menu_events.read() {
        if !event.clicked() {
            continue;
        }
        let source = event.source();
        if source == edit_menu.undo {
            undo_request.write(UndoRequest);
        } else if source == edit_menu.redo {
            redo_request.write(RedoRequest);
        } else if source == edit_menu.delete {
            if let Some(selected) = selection.0 {
                delete.write(Delete::new(selected));
            }
        }
    }
}

#[derive(Default)]
pub struct EditMenuPlugin {}

impl Plugin for EditMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EditMenuItems>().add_systems(
            Update,
            (update_edit_menu_state, handle_edit_menu_events)
                .run_if(AppState::in_displaying_mode()),
        );
    }
}

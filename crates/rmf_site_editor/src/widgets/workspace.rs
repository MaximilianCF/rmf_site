/*
 * Copyright (C) 2024 Open Source Robotics Foundation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
*/

use crate::widgets::{FileMenu, MenuDisabled, MenuEvent, MenuItem, TextMenuItem};
use crate::{AppState, CreateNewWorkspace, WorkspaceLoader, WorkspaceSaver};
use bevy::{ecs::hierarchy::ChildOf, prelude::*};

#[derive(Default)]
pub struct WorkspaceMenuPlugin {}

impl Plugin for WorkspaceMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorkspaceMenu>().add_systems(
            Update,
            handle_workspace_menu_events.run_if(AppState::in_displaying_mode()),
        );
    }
}

#[derive(Resource)]
pub struct WorkspaceMenu {
    new: Entity,
    open: Entity,
    save: Entity,
    save_as: Entity,
}

impl FromWorld for WorkspaceMenu {
    fn from_world(world: &mut World) -> Self {
        let file_menu = world.resource::<FileMenu>().get();

        let new = world
            .spawn((
                MenuItem::Text(TextMenuItem::new("New").shortcut("Ctrl-N")),
                ChildOf(file_menu),
            ))
            .id();
        let open = world
            .spawn((
                MenuItem::Text(TextMenuItem::new("Open").shortcut("Ctrl-O")),
                ChildOf(file_menu),
            ))
            .id();

        // Separator between open/save
        world.spawn((MenuItem::Separator, ChildOf(file_menu)));

        let save = world
            .spawn((
                MenuItem::Text(TextMenuItem::new("Save").shortcut("Ctrl-S")),
                ChildOf(file_menu),
            ))
            .id();
        let save_as = world
            .spawn((
                MenuItem::Text(TextMenuItem::new("Save As").shortcut("Ctrl-Shift-S")),
                ChildOf(file_menu),
            ))
            .id();

        // Saving/opening is not enabled in wasm
        if cfg!(target_arch = "wasm32") {
            world.entity_mut(new).insert(MenuDisabled);
            world.entity_mut(open).insert(MenuDisabled);
            world.entity_mut(save).insert(MenuDisabled);
            world.entity_mut(save_as).insert(MenuDisabled);
        }
        Self {
            new,
            open,
            save,
            save_as,
        }
    }
}

fn handle_workspace_menu_events(
    mut menu_events: EventReader<MenuEvent>,
    workspace_menu: Res<WorkspaceMenu>,
    mut workspace_saver: WorkspaceSaver,
    mut workspace_loader: WorkspaceLoader,
    mut new_workspace: EventWriter<CreateNewWorkspace>,
) {
    for event in menu_events.read() {
        if !event.clicked() {
            continue;
        }
        let source = event.source();
        if source == workspace_menu.new {
            new_workspace.write(CreateNewWorkspace);
        } else if source == workspace_menu.open {
            workspace_loader.load_from_dialog();
        } else if source == workspace_menu.save {
            workspace_saver.save_to_default_file();
        } else if source == workspace_menu.save_as {
            workspace_saver.save_to_dialog();
        }
    }
}

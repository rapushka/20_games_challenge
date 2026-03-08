use bevy::input::common_conditions::input_toggle_active;
use crate::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                EguiPlugin::default(),
                WorldInspectorPlugin::new()
                    .run_if(input_toggle_active(false, constants::input::TOGGLE_DEBUG)),
            ))
        ;
    }
}
use bevy::input::common_conditions::input_toggle_active;
use crate::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::collision_detection::debug_colliders;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                EguiPlugin::default(),
                WorldInspectorPlugin::new()
                    .run_if(debug_mode_condition()),
            ))

            .add_systems(Update, (
                (
                    debug_colliders,
                    stop_time,
                ).run_if(debug_mode_condition()),
            ))
        ;
    }
}

fn debug_mode_condition() -> impl FnMut(Res<ButtonInput<KeyCode>>) -> bool + Clone {
    input_toggle_active(false, constants::input::TOGGLE_DEBUG)
}

fn stop_time(
    mut time: ResMut<Time<Virtual>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(constants::input::DEBUG_STOP_TIME) {
        if time.is_paused() {
            time.unpause();
        } else {
            time.pause();
        }
    }
}
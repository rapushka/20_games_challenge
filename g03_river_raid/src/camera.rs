use crate::position::WorldPosition;
use crate::prelude::*;
use crate::utils::ResultExt;

#[derive(Component)]
pub struct CameraFollowTarget;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Bootstrap), (
                spawn_camera,
            ))

            .add_systems(Update, (
                camera_follow,
            ))
        ;
    }
}

fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn((
        Camera2d,
        WorldPosition::ZERO,
    ));
}

fn camera_follow(
    cameras: Query<&mut WorldPosition, With<Camera>>,
    targets: Query<&WorldPosition, (With<CameraFollowTarget>, Without<Camera>)>,
) {
    let result = targets.single();
    let Some(target) = result.zero_or_one() else {
        return;
    };

    for mut position in cameras {
        *position = **target;
    }
}
use crate::prelude::*;

const DEAD_FALL_SPEED: f32 = 200.0;

#[derive(Component)]
pub struct Dead;

#[derive(Component)]
pub struct FallAnimation {
    duration: f32,
    passed_duration: f32,
    initial_y: f32,
    curve: CubicCurve<f32>,
}

impl FallAnimation {
    pub fn progress(&self) -> f32 {
        self.passed_duration / self.duration
    }

    pub fn current_y(&self) -> f32 {
        let t = self.progress();
        (self.curve.position(t) * 20.0) + self.initial_y
    }
}

pub fn start_fall_animation(
    mut commands: Commands,
    characters: Query<(Entity, &Transform), (With<Dead>, Without<FallAnimation>)>,
) {
    for (character, transform) in characters {
        commands.entity(character)
            .insert(FallAnimation {
                duration: 0.3,
                passed_duration: 0.0,
                initial_y: transform.translation.y,
                curve: create_curve(),
            });
    }
}

pub fn fall_dead_characters(
    characters: Query<(&mut Transform, &mut FallAnimation), With<Dead>>,
    time: Res<Time>,
) {
    let delta_time = time.delta_secs();

    for (mut transform, mut fall_animation) in characters {
        transform.translation.y = fall_animation.current_y();
        fall_animation.passed_duration += delta_time;
    }
}

pub fn despawn_corpse_offscreen(
    mut commands: Commands,
    characters: Query<(Entity, &Transform), With<Dead>>,
) {
    for (character, transform) in characters {
        if transform.translation.y < -constants::CANVAS_HALF_SIZE.y - 50.0 {
            commands.entity(character).despawn();
            info!("DESPAWNED!");
        }
    }
}

fn create_curve() -> CubicCurve<f32> {
    // easeInBack = (0.36, 0.0, 0.66, -0.56)
    // 0.29, -0.89, 0.88, 0.17
    CubicBezier::new([[0.08, 1.03, 0.39, -1.02]]).to_curve().unwrap()
}
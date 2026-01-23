use crate::paddle::{Paddle, Side};
use crate::prelude::*;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Movement {
    y: f32,
}

impl Movement {
    pub fn y(&self) -> f32 {
        self.y
    }
}

pub fn read_players_input(
    mut commands: Commands,
    paddles: Query<(Entity, &Side), With<Paddle>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    read_input(&mut commands, &paddles, &keyboard, Side::Left, KeyCode::KeyW, KeyCode::KeyS);
    read_input(&mut commands, &paddles, &keyboard, Side::Right, KeyCode::ArrowUp, KeyCode::ArrowDown);
}

fn read_input(
    commands: &mut Commands,
    paddles: &Query<(Entity, &Side), With<Paddle>>,
    keyboard: &Res<ButtonInput<KeyCode>>,
    target_side: Side,
    key_up: KeyCode,
    key_down: KeyCode,
) {
    let up = keyboard.pressed(key_up);
    let down = keyboard.pressed(key_down);

    let vertical = up as i8 - down as i8;
    let direction = vertical as f32;

    for (e, side) in paddles {
        if *side != target_side {
            continue;
        }

        let mut entity = commands.entity(e);
        if direction != 0.0 {
            entity.insert(Movement { y: direction });
        } else {
            entity.remove::<Movement>();
        }
    }
}
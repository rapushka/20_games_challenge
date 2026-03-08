pub mod player {
    pub const HORIZONTAL_SPEED: f32 = 250.0;
    pub const FLY_SPEED: f32 = 250.0;

    pub const ACCELERATION_MULT: f32 = 1.5;
    pub const DECELERATION_MULT: f32 = 0.5;
}

pub mod input {
    use bevy::prelude::KeyCode;

    pub const MOVE_LEFT: [KeyCode; 2] = [KeyCode::ArrowLeft, KeyCode::KeyA];
    pub const MOVE_RIGHT: [KeyCode; 2] = [KeyCode::ArrowRight, KeyCode::KeyD];
    pub const MOVE_ACCELERATE: [KeyCode; 2] = [KeyCode::ArrowUp, KeyCode::KeyW];
    pub const MOVE_DECELERATE: [KeyCode; 2] = [KeyCode::ArrowDown, KeyCode::KeyS];

    pub const TOGGLE_DEBUG: KeyCode = KeyCode::Backquote;
    pub const DEBUG_STOP_TIME: KeyCode = KeyCode::KeyT;
}
use crate::order::UpdateOrder;
use crate::prelude::*;

#[derive(Message)]
pub struct Destroy(pub Entity);

pub struct DestructionPlugin;

impl Plugin for DestructionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_message::<Destroy>()

            .add_systems(Update, (
                despawn_destroyed,
            ).in_set(UpdateOrder::Cleanups))
        ;
    }
}

fn despawn_destroyed(
    mut commands: Commands,
    mut messages: MessageReader<Destroy>,
) {
    for message in messages.read() {
        commands.entity(message.0)
            .try_despawn();
    }
}
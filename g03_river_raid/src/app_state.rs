use crate::prelude::*;

#[derive(States, Default, PartialEq, Clone, Eq, Hash, Debug)]
pub enum AppState {
    #[default]
    Bootstrap,
    Initialize,
    Playing,
}
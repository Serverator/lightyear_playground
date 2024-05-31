use bevy::prelude::*;
use lightyear::prelude::*;
use serde::{ Serialize, Deserialize };

pub struct ProtocolPlugin;
impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        app.register_component::<CursorPosition>(ChannelDirection::Bidirectional);
        app.register_component::<Owner>(ChannelDirection::ServerToClient);

        app.register_type::<CursorPosition>();
        app.register_type::<Owner>();
    }
}

#[derive(Component, Debug, Reflect, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Owner(pub ClientId);

#[derive(Component, Debug, Default, Clone, Copy, Reflect, Serialize, Deserialize, PartialEq)]
pub struct CursorPosition {
    pub position: Vec2,
}

impl Linear for CursorPosition {
    fn lerp(start: &Self, other: &Self, t: f32) -> Self {
        CursorPosition { position: Vec2::lerp(start.position, other.position, t) }
    }
}

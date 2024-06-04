use bevy::prelude::*;
use lightyear::prelude::*;
use rand::Rng;
use serde::{ Serialize, Deserialize };

pub struct ProtocolPlugin;
impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<VeryLargeMessage>(ChannelDirection::Bidirectional);

        app.add_channel::<UnorderedReliableChannel>(ChannelSettings {
            direction: ChannelDirection::Bidirectional,
            mode: ChannelMode::UnorderedReliable(ReliableSettings::default()),
            ..default()
        });
    }
}

#[derive(Channel)]
pub struct UnorderedReliableChannel;

#[derive(Serialize, Deserialize)]
pub struct VeryLargeMessage {
    pub data: Vec<u8>,
}

impl VeryLargeMessage {
    pub fn generate(size: usize) -> Self {
        let mut rand = rand::thread_rng();
        let data = (0..size).map(|_| rand.gen::<u8>()).collect();
        VeryLargeMessage { data }
    }
}
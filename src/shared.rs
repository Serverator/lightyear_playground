use bevy::prelude::*;
use lightyear::prelude::*;
use std::time::Duration;

pub const TICKRATE: f64 = 60.0;
pub const DEFAULT_PORT: u16 = 27007;

pub struct MySharedPlugin;
impl Plugin for MySharedPlugin {
    fn build(&self, app: &mut App) {
        let config = SharedConfig {
            mode: Mode::Separate,
            tick: TickConfig {
                tick_duration: Duration::from_secs_f64(1.0 / TICKRATE),
            },
            client_send_interval: Duration::ZERO,
            server_send_interval: Duration::ZERO,
        };

        app.add_plugins(SharedPlugin { config })
		   .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
use bevy::prelude::*;
use client::{Confirmed, Interpolated};
use lightyear::prelude::*;
use std::time::Duration;
use crate::protocol::CursorPosition;

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
		   .add_systems(Startup, spawn_camera)
		   .add_systems(Update, draw_cursors);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

/// Red cursor - local cursor
/// Cyan cursor - replicated cursor
fn draw_cursors(
	mut gizmos: Gizmos,
    local_cursor: Query<&CursorPosition, (Without<Replicated>, Without<Confirmed>, Without<Interpolated>)>,
	replicated_cursors: Query<&CursorPosition, With<Replicated>>,
) {
	for cursor in local_cursor.iter() {
		gizmos.circle_2d(cursor.position, 20.0, Color::RED);
	}

    for cursor in replicated_cursors.iter() {
		gizmos.circle_2d(cursor.position, 18.0, Color::CYAN);
	}
}

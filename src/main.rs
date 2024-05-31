#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod client;
mod protocol;
mod server;
mod shared;

fn main() {
    let is_server = std::env::args().any(|arg| arg == "server");
    let is_client = std::env::args().any(|arg| arg == "client");

    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin { 
            primary_window: Some(Window { 
                title: String::from(if is_server {"Server"} else {"Client"}),
                ..default()
            }),
            ..default()
        }),
        WorldInspectorPlugin::default(),
        shared::MySharedPlugin,
    ));

    if is_client {
        app.add_plugins(client::MyClientPlugin);
    }

    if is_server {
        app.add_plugins(server::MyServerPlugin);
    }

    app.add_plugins(protocol::ProtocolPlugin);

    app.run();
}


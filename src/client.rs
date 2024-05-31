use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::bevy_egui::EguiContext;
use lightyear::{connection::netcode::PRIVATE_KEY_BYTES, prelude::{client::*, * }};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use crate::{protocol::CursorPosition, shared::DEFAULT_PORT};
use rand::Rng;
use bevy_inspector_egui::egui;

pub struct MyClientPlugin;
impl Plugin for MyClientPlugin {
    fn build(&self, app: &mut App) {
		let config = client_config();

        app .add_plugins(ClientPlugins::new(config))
			.add_systems(Startup, spawn_local_cursor)
        	.add_systems(Update, (update_local_cursor_position, draw_connection_ui))
			// Connect the client
			.insert_resource(NextState(Some(NetworkingState::Connecting)));
    }
}

fn spawn_local_cursor(mut commands: Commands) {
    commands.spawn((
        Name::new("Cursor"),
        CursorPosition::default(),
        client::Replicate::default(),
    ));
}

fn draw_connection_ui(
	mut egui_context: Query<&mut EguiContext>,
	mut commands: Commands,
	client_state: Res<State<client::NetworkingState>>,
) {
    let mut egui_context = egui_context.single_mut();

	let window = egui::Window::new("Connection window")
		.enabled(true)
		.collapsible(true);

	window.show(egui_context.get_mut(), |ui| {
		ui.set_max_width(200.0);

		match client_state.get() {
			client::NetworkingState::Disconnected => {
				if ui.button("Connect").clicked() {
					commands.connect_client();
				}
			}
			client::NetworkingState::Connecting => {
				ui.add_enabled_ui(false, |ui| ui.button("Connecting"));
			}
			client::NetworkingState::Connected => {
				if ui.button("Disconnect").clicked() {
					commands.disconnect_client();
				}
			}
		}
	});
}

fn update_local_cursor_position(
    mut local_cursor: Query<&mut CursorPosition, (Without<Replicated>, Without<Confirmed>, Without<Interpolated>)>,
    camera: Query<(&Camera, &GlobalTransform)>,
	window: Query<&Window, With<PrimaryWindow>>,
) {
	let (camera, camera_transform) = camera.single();
	let window = window.single();
	let mut cursor = local_cursor.single_mut();

	if let Some(world_pos) = window
		.cursor_position()
		.and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor)) 
	{
		cursor.set_if_neq(CursorPosition { position: world_pos });
	};
}


fn client_config() -> ClientConfig {
	let server_addr =  std::net::SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), DEFAULT_PORT));
	
	let io = IoConfig {
		//transport: ClientTransport::UdpSocket(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0))),
		transport: ClientTransport::WebTransportClient {
			server_addr,
			client_addr: SocketAddr::V4(
		   		SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0),
			),
		},
		..default()
	};

	let auth = Authentication::Manual {
		server_addr,
		client_id: rand::thread_rng().gen_range(0..10000),
		private_key: [0; PRIVATE_KEY_BYTES],
		protocol_id: 0,
	};

	ClientConfig {
		net: NetConfig::Netcode {
			config: NetcodeConfig::default(),
			auth,
			io,
		},
		..default()
	}
}
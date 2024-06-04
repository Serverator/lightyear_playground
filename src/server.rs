use bevy::prelude::*;
use lightyear::prelude::{*, server::* };
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use crate::{protocol::{UnorderedReliableChannel, VeryLargeMessage}, shared::DEFAULT_PORT};

pub struct MyServerPlugin;
impl Plugin for MyServerPlugin {
    fn build(&self, app: &mut App) {
		let config = server_config();

        app.add_plugins(ServerPlugins::new(config))
           .add_systems(PreUpdate, send_messages_on_connection.after(MainSet::EmitEvents))
		   // Start the server
		   .insert_resource(NextState(Some(server::NetworkingState::Started)));
    }
}

fn send_messages_on_connection(
    mut connect_events: EventReader<ConnectEvent>,
    mut connection: ResMut<ConnectionManager>,
) {
    for connect in connect_events.read() {
        let message = VeryLargeMessage::generate(300000);
        connection.send_message::<UnorderedReliableChannel,_>(connect.client_id, &message).unwrap();
        info!("Large message sent: {:?}...", &message.data[..10]);
    }
}

fn server_config() -> ServerConfig {
	let io = IoConfig {
        // transport: ServerTransport::UdpSocket(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), DEFAULT_PORT))),
		transport: ServerTransport::WebTransportServer { 
			server_addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), DEFAULT_PORT)),
			certificate: Identity::self_signed(["localhost"]).unwrap()
		},
		..default()
	};

	let net = NetConfig::Netcode {
		io,
		config: NetcodeConfig::default(),
	};

	ServerConfig {
		net: vec![net],
		..default()
	}
}

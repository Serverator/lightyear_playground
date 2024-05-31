use bevy::prelude::*;
use lightyear::prelude::{*, server::* };
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use crate::{protocol::{CursorPosition, Owner}, shared::DEFAULT_PORT};

pub struct MyServerPlugin;
impl Plugin for MyServerPlugin {
    fn build(&self, app: &mut App) {
		let config = server_config();

        app.add_plugins(ServerPlugins::new(config))
           .add_systems(PreUpdate, replicate_cursors.in_set(ServerReplicationSet::ClientReplication))
		   // Start the server
		   .insert_resource(NextState(Some(server::NetworkingState::Started)));
    }
}


fn replicate_cursors(
    mut commands: Commands,
    cursors: Query<(Entity, &Replicated), (With<CursorPosition>, Added<Replicated>)>,
) {
    for (entity, replicated) in cursors.iter() {
        let client_id = replicated.client_id();
        let mut entity = commands.entity(entity);

        entity.insert((
            Owner(client_id),
            server::Replicate {
                target: ReplicationTarget {
                    target: NetworkTarget::All,
                },
                ..default()
            },
        ));
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

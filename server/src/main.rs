#![allow(dead_code)]
use std::thread;
use websocket::sync::Server;
use websocket::OwnedMessage;
use protocol::message::Message;
use protocol::entity::*;
mod map;
use crate::map::*;

fn main() {
	let server = Server::bind("localhost:2794").unwrap();

	for request in server.filter_map(Result::ok) {
		// Spawn a new thread for each connection.
		thread::spawn(|| {

			let mut client = request.use_protocol("dungeon_game_protocol").accept().unwrap();

			let ip = client.peer_addr().unwrap();
			let map = Map::new();
			let player = Entity::spawn_player("undefined".to_string());

			println!("Connection from {}", ip);

			let (mut receiver, mut sender) = client.split().unwrap();

			for message in receiver.incoming_messages() {
				let message = message.expect("can't read message");
				

				match message {
					OwnedMessage::Close(_) => {
						let message = OwnedMessage::Close(None);
						sender.send_message(&message).expect("can't send message after close");
						println!("Client {} disconnected", ip);
						return;
					}
					OwnedMessage::Ping(ping) => {
						let message = OwnedMessage::Pong(ping);
						sender.send_message(&message).expect("can't send response to ping");
					}
					OwnedMessage::Text(data) => {
						if let Ok(message) = Message::decode(data) {
							match message {
								Message::Init{username, screen_width, screen_height, password} => {
									println!("{} has connected", username);
									sender.send_message(&OwnedMessage::Text(Message::CreateEntity(Entity::spawn_player(username)).encode())).expect("can't send this");
									sender.send_message(&OwnedMessage::Text(Message::Chunk(map.get_chunk(player.get_coords().0, player.get_coords().1)).encode())).expect("can't send this");
								},
								message => {
									println!("{:?}", message);
								},
							}
						} else {
							println!("can't deserialize message");
						}
					},
					_ => {
						println!("unknwon message format");
					}
				}
			}
		});
	}
}
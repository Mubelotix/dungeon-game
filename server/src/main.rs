#![allow(dead_code)]
use std::thread;
use websocket::sync::Server;
use websocket::OwnedMessage;
use protocol::message::Message;
use protocol::entity::*;
mod map;
use crate::map::*;
use std::sync::Arc;
use std::sync::Mutex;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
	let server = Server::bind("localhost:2794").unwrap();
	let map: Arc<Mutex<Map>> = Arc::new(Mutex::new(Map::new()));
	let entities: Arc<Mutex<Vec<Arc<Mutex<Entity>>>>> = Arc::new(Mutex::new(Vec::new()));

	for request in server.filter_map(Result::ok) {
		let map = Arc::clone(&map);
		let entities = Arc::clone(&entities);

		thread::spawn(move || {
			let client = request.use_protocol("dungeon_game_protocol").accept().unwrap();

			let ip = client.peer_addr().unwrap();
			
			let mut chunk_left_top_coords: (u64, u64) = (9_223_372_036_854_775_808 - 3 * 8, 9_223_372_036_854_775_808 - 2 * 8);
			let player = Arc::new(Mutex::new(Entity::spawn_player("undefined".to_string())));

			let (mut receiver, mut sender) = client.split().unwrap();

			for message in receiver.incoming_messages() {
				let player = Arc::clone(&player);
				let message = message.expect("can't read message");
				
				match message {
					OwnedMessage::Close(_) => {
						let message = OwnedMessage::Close(None);
						sender.send_message(&message).expect("can't send message after close");
						println!("{} disconnected", player.lock().unwrap().get_name());
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
								    { player.lock().unwrap().set_entity_name(username); }
									sender.send_message(&OwnedMessage::Text(Message::CreateEntity(player.lock().unwrap().clone()).encode())).expect("can't send this");
									entities.lock().unwrap().push(player);

									let map = map.lock().unwrap();
									for i in 0..6 {
										for j in 0..4 {
											sender.send_message(&OwnedMessage::Text(Message::Chunk(map.get_chunk(chunk_left_top_coords.0 + i * 8, chunk_left_top_coords.1 + j * 8)).encode())).expect("can't send chunk");
										}
									}
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
						println!("unexpected message format");
					}
				}
			}
		});
	}
}
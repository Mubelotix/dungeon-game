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
use std::sync::mpsc::channel;
use std::time::Duration;
use protocol::block::Block;
use protocol::block::BlockCode;
use protocol::block::Orientation;

fn main() {
	let server = Server::bind("localhost:2794").unwrap();
	let map: Arc<Mutex<Map>> = Arc::new(Mutex::new(Map::new()));
	let entities: Arc<Mutex<Vec<Arc<Mutex<Entity>>>>> = Arc::new(Mutex::new(Vec::new()));
	

	for request in server.filter_map(Result::ok) {
		let map = Arc::clone(&map);
		let entities = Arc::clone(&entities);

		{
			let mut map = map.lock().unwrap();
			map[(9_223_372_036_854_775_808, 9_223_372_036_854_775_808)] = Block::new(BlockCode::SimpleWall, Orientation::Up);
		}

		thread::spawn(move || {
			let client = request.use_protocol("dungeon_game_protocol").accept().unwrap();

			let ip = client.peer_addr().unwrap();
			
			let mut chunk_left_top_coords: (u64, u64) = (9_223_372_036_854_775_808 - 3 * 8, 9_223_372_036_854_775_808 - 2 * 8);
			let player = Arc::new(Mutex::new(Entity::spawn_player("undefined".to_string())));

			let (mut receiver, mut sender) = client.split().unwrap();

			let (tx, rx) = channel::<OwnedMessage>();
			thread::spawn(move || {
				loop {
					let message = rx.recv().unwrap();
					sender.send_message(&message).expect("can't send throught websocket");
				}
			});

			let tx2 = tx.clone();
			thread::spawn(move || {
				loop {
					thread::sleep(Duration::from_millis(16));
					tx2.send(OwnedMessage::Text(Message::Tick.encode())).expect("can't send throught channel");
				}
			});

			for message in receiver.incoming_messages() {
				let player = Arc::clone(&player);
				let message = message.expect("can't read message");
				
				match message {
					OwnedMessage::Close(_) => {
						let message = OwnedMessage::Close(None);
						tx.send(message).unwrap();
						println!("{} disconnected", player.lock().unwrap().get_name());
						return;
					}
					OwnedMessage::Ping(ping) => {
						let message = OwnedMessage::Pong(ping);
						tx.send(message).unwrap();
					}
					OwnedMessage::Text(data) => {
						if let Ok(message) = Message::decode(data) {
							match message {
								Message::Init{username, screen_width, screen_height, password} => {
									println!("{} has connected", username);
								    { player.lock().unwrap().set_entity_name(username); }
									tx.send(OwnedMessage::Text(Message::CreateEntity(player.lock().unwrap().clone()).encode())).unwrap();
									entities.lock().unwrap().push(player);

									let map = map.lock().unwrap();
									for i in 0..6 {
										for j in 0..4 {
											tx.send(OwnedMessage::Text(Message::Chunk(map.get_chunk(chunk_left_top_coords.0 + i * 8, chunk_left_top_coords.1 + j * 8)).encode())).unwrap();
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
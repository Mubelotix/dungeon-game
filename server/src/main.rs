#![allow(dead_code)]
use std::thread;
use websocket::sync::Server;
use websocket::OwnedMessage;
use protocol::message::Message;
use protocol::entity::*;
use protocol::map::*;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::channel;
use std::time::Duration;
use protocol::block::Block;
use protocol::block::BlockCode;
use protocol::block::Orientation;

const CENTER_POINT: u64 = 9_223_372_036_854_775_808;

fn main() {
	let server = Server::bind("localhost:2794").unwrap();
	let map: Arc<Mutex<Map>> = Arc::new(Mutex::new(Map::new()));
	let entities: Arc<Mutex<Vec<Arc<Mutex<Entity>>>>> = Arc::new(Mutex::new(Vec::new()));

	{
		let mut map = map.lock().unwrap();
		map[(CENTER_POINT + 70, CENTER_POINT)] = Block::new(BlockCode::SimpleWall, Orientation::Up);
		map[(9_223_372_036_854_775_808, 9_223_372_036_854_775_807)] = Block::new(BlockCode::SimpleWall, Orientation::Up);
		map[(9_223_372_036_854_775_808, 9_223_372_036_854_775_806)] = Block::new(BlockCode::SimpleWall, Orientation::Up);
		map[(9_223_372_036_854_775_808, 9_223_372_036_854_775_805)] = Block::new(BlockCode::SimpleWall, Orientation::Up);
		map[(9_223_372_036_854_775_807, 9_223_372_036_854_775_805)] = Block::new(BlockCode::SimpleWall, Orientation::Up);
		map[(9_223_372_036_854_775_806, 9_223_372_036_854_775_805)] = Block::new(BlockCode::SimpleWall, Orientation::Up);
		map[(9_223_372_036_854_775_805, 9_223_372_036_854_775_805)] = Block::new(BlockCode::SimpleWall, Orientation::Up);
		map[(9_223_372_036_854_775_804, 9_223_372_036_854_775_805)] = Block::new(BlockCode::SimpleWall, Orientation::Up);
	}
	
	for request in server.filter_map(Result::ok) {
		let map = Arc::clone(&map);
		let entities = Arc::clone(&entities);

		thread::spawn(move || {
			let client = request.use_protocol("dungeon_game_protocol").accept().unwrap();
			let _ip = client.peer_addr().unwrap();
			let mut loaded_chunks_top_left: (u64, u64) = (9_223_372_036_854_775_808 - 4 * 8, 9_223_372_036_854_775_808 - 2 * 8);
			let player = Arc::new(Mutex::new(Entity::spawn_player("undefined".to_string())));
			let player_id: u64 = {player.lock().unwrap().get_id()};

			let (mut receiver, mut sender) = client.split().unwrap();

			{
				let mut entities = entities.lock().unwrap();
				entities.push(Arc::clone(&player));
			}

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
								Message::Init{username, screen_width: _, screen_height: _, password: _} => {
									println!("{} has connected", username);
								    { player.lock().unwrap().set_entity_name(username); }
									tx.send(OwnedMessage::Text(Message::CreateEntity(player.lock().unwrap().clone()).encode())).unwrap();
									entities.lock().unwrap().push(player);

									let map = map.lock().unwrap();
									for i in 0..8 {
										for j in 0..4 {
											tx.send(OwnedMessage::Text(Message::Chunk(map.get_chunk(loaded_chunks_top_left.0 + i * 8, loaded_chunks_top_left.1 + j * 8)).encode())).unwrap();
										}
									}
								},
								Message::MoveEntity{id, direction} => {
									if id == player_id {
										let map = map.lock().unwrap();
										let mut player = player.lock().unwrap();
										if !map[player.get_coords_after_eventual_move(direction)].is_solid() {
											player.move_in_direction(direction);
										} else {
											println!("can't move in a solid block");
											tx.send(OwnedMessage::Text(Message::TpEntity{id: player.get_id(), x: player.get_coords().0, y: player.get_coords().1, x2: player.get_position_in_block().0, y2: player.get_position_in_block().1}.encode())).unwrap();
										}
										let player_chunk_coords = (player.get_coords().0 - (player.get_coords().0 % 8), player.get_coords().1 - (player.get_coords().1 % 8));
										let needed_chunks_top_left = (player_chunk_coords.0 - 4*8, player_chunk_coords.1 - 2*8);

										println!("{:?}", player.get_readable_coords());

										// if we must load chunks to left
										if needed_chunks_top_left.0 < loaded_chunks_top_left.0 {
											for i in 0..(loaded_chunks_top_left.0 - needed_chunks_top_left.0) / 8 {
												for j in 0..4 {
													tx.send(OwnedMessage::Text(Message::Chunk(map.get_chunk(needed_chunks_top_left.0 + i * 8, loaded_chunks_top_left.1 + j * 8)).encode())).unwrap();
													tx.send(OwnedMessage::Text(Message::UnloadChunk{x: needed_chunks_top_left.0 + i * 8 + 64, y: loaded_chunks_top_left.1 + j * 8}.encode())).unwrap();
												}
											}
											loaded_chunks_top_left.0 = needed_chunks_top_left.0;
										} else if needed_chunks_top_left.0 > loaded_chunks_top_left.0 {
											for i in 0..(needed_chunks_top_left.0 - loaded_chunks_top_left.0) / 8 {
												for j in 0..4 {
													tx.send(OwnedMessage::Text(Message::Chunk(map.get_chunk(loaded_chunks_top_left.0 + i * 8 + 64, loaded_chunks_top_left.1 + j * 8)).encode())).unwrap();
													tx.send(OwnedMessage::Text(Message::UnloadChunk{x: loaded_chunks_top_left.0 + i * 8, y: loaded_chunks_top_left.1 + j * 8}.encode())).unwrap();
												}
											}
											loaded_chunks_top_left.0 = needed_chunks_top_left.0;
										}

										// if we must load chunks to top
										if needed_chunks_top_left.1 < loaded_chunks_top_left.1 {
											for i in 0..8 {
												for j in 0..(loaded_chunks_top_left.1 - needed_chunks_top_left.1) / 8 {
													tx.send(OwnedMessage::Text(Message::Chunk(map.get_chunk(loaded_chunks_top_left.0 + i * 8, needed_chunks_top_left.1 + j * 8)).encode())).unwrap();
													tx.send(OwnedMessage::Text(Message::UnloadChunk{x: loaded_chunks_top_left.0 + i * 8, y: needed_chunks_top_left.1 + j * 8 + 64}.encode())).unwrap();
												}
											}
											loaded_chunks_top_left.1 = needed_chunks_top_left.1;
										} else if needed_chunks_top_left.1 > loaded_chunks_top_left.1 {
											for i in 0..8 {
												for j in 0..(needed_chunks_top_left.1 - loaded_chunks_top_left.1) / 8 {
													tx.send(OwnedMessage::Text(Message::Chunk(map.get_chunk(loaded_chunks_top_left.0 + i * 8, loaded_chunks_top_left.1 + 64 + j * 8)).encode())).unwrap();
													tx.send(OwnedMessage::Text(Message::UnloadChunk{x: loaded_chunks_top_left.0 + i * 8, y: loaded_chunks_top_left.1 + j * 8}.encode())).unwrap();
												}
											}
											loaded_chunks_top_left.1 = needed_chunks_top_left.1;
										}
									} else {
										println!("attempt to move an unowned entity");
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
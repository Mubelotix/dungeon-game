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
use chrono::Local;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::io;
use std::process;

const CENTER_POINT: u64 = 9_223_372_036_854_775_808;

fn main() {
	let server = match Server::bind("localhost:2794") {
		Ok(server) => server,
		Err(error) if error.kind() == std::io::ErrorKind::AddrInUse => {
			println!("The port 2794 is already in use.");
			
			loop {
				println!("You can: exit program (E); try another port (T); retry (R)");
				let mut choice = String::new();
				io::stdin().read_line(&mut choice).expect("expected stdin stream");
				
				match choice.trim() {
					"E" | "e" => {
						process::exit(0x0100);
					}
					"T" | "t" => {
						println!("Enter a new port:");

						let mut port = String::new();
						io::stdin().read_line(&mut port).expect("expected stdin stream");

						if let Ok(port) = port.trim().parse::<u16>() {
							match Server::bind(format!("localhost:{}", port)) {
								Ok(server) => {
									break server;
								},
								Err(error) if error.kind() == std::io::ErrorKind::AddrInUse => {
									println!("this port is in use too!");
									continue;
								},
								Err(error) => {
									panic!("An error occured when starting server: {}.", error);
								}
							}
						} else {
							println!("Please enter a valid port number");
							continue;
						}
					}
					"R" | "r" => {
						match Server::bind(format!("localhost:2794")) {
							Ok(server) => {
								break server;
							},
							Err(error) if error.kind() == std::io::ErrorKind::AddrInUse => {
								panic!("The port 2794 is still used.");
							},
							Err(error) => {
								panic!("An error occured when starting server: {}.", error);
							}
						}
					}
					_ => {
						println!("Please enter a valid choice.");
						continue;
					}
				};
			}
		},
		Err(error) => panic!("An error occured when starting server: {}.", error),
	};
	let map: Arc<Mutex<Map>> = Arc::new(Mutex::new(Map::new()));
	let entities: Arc<Mutex<HashMap<u64, Arc<Mutex<Entity>>>>> = Arc::new(Mutex::new(HashMap::new()));

	{
		let mut map = map.lock().unwrap();
		for x in 0..12 {
			map[(9_223_372_036_854_775_810 + x, 9_223_372_036_854_775_807)] = Block::new(BlockCode::SimpleWall, Orientation::Up);
		}

		for y in 0..8 {
			map[(9_223_372_036_854_775_810, 9_223_372_036_854_775_807 + y)] = Block::new(BlockCode::SimpleWall, Orientation::Up);
			map[(9_223_372_036_854_775_810 + 11, 9_223_372_036_854_775_807 + y)] = Block::new(BlockCode::SimpleWall, Orientation::Up);
		}
	}

	let entities2 = Arc::clone(&entities);
	thread::spawn(move || {
		
		
		loop {
			let mut input = String::new();
			io::stdin().read_line(&mut input).expect("expected stdin stream");
			let words: Vec<&str> = input.trim().split(' ').collect();
			if words.len() > 0 {
				match words[0] {
					"help" => println!("COMMANDS LIST:\n\
						- help => display this page\n\
						- tp [id] [x] [y] => teleport an entity where you want\n\
						- list_entities player => list the connected players"),
					"tp" => {
						if words.len() != 4 {
							println!("tp command require 3 arguments");
						} else {
							if let Ok(id) = words[1].parse::<u64>() {
								if let Ok(x) = words[2].parse::<u64>() {
									if let Ok(y) = words[3].parse::<u64>() {
										let mut entities = entities2.lock().unwrap();
										if let Entry::Occupied(entity) = entities.entry(id) {
											let entity = Arc::clone(&entity.get());
											let mut entity = entity.lock().unwrap();
											entity.set_coords((x, y));
											println!("entity has been teleported successfully");
										} else {
											println!("entity does not exist. check existing entity with the command list");
										}
									} else {
										println!("fourth argument must be a number");
									}
								} else {
									println!("third argument must be a number");
								}
							} else {
								println!("second argument must be a number");
							}
						}
					},
					"list_entities" => {
						if words.len() != 2 {
							println!("list_entities command require 1 argument");
						} else if words[1] == "players" {
							let mut connected_players: Vec<(u64, _)> = Vec::new();
							
							let entities = entities2.lock().unwrap();
							for (id, entity) in entities.iter() {
								let entity_arc = Arc::clone(&entity);
								let entity_arc2 = Arc::clone(&entity);
								let entity = entity_arc.lock().unwrap();
								if entity.get_type() == EntityType::Player {
									connected_players.push((*id, entity_arc2));
								}
							}

							println!("{} players connected", connected_players.len());

							for (id, entity) in connected_players {
								let entity = entity.lock().unwrap();
								println!("{} (id: {})", entity.get_name(), id);
							}
						} else {
							println!("unkow option: {}", words[1]);
						}
					},
					_ => println!("unknow command; type help to get the full list of commands"),

				}
			}
		}
	});

	println!("\x1B[90m[{}]\x1B[0m Server is running. Type help to see available commands.", Local::now().format("%T"));
	
	
	for request in server.filter_map(Result::ok) {
		let map = Arc::clone(&map);
		let entities = Arc::clone(&entities);

		thread::spawn(move || {
			let client = request.use_protocol("dungeon_game_protocol").accept().unwrap();
			let _ip = client.peer_addr().unwrap();
			let mut loaded_chunks_top_left: (u64, u64) = (9_223_372_036_854_775_808 - 4 * 8, 9_223_372_036_854_775_808 - 2 * 8);
			let player = Arc::new(Mutex::new(Entity::spawn_player("undefined".to_string())));
			let player2 = Arc::clone(&player);
			let player_id: u64 = {player.lock().unwrap().get_id()};

			let (mut receiver, mut sender) = client.split().unwrap();

			{
				let mut entities = entities.lock().unwrap();
				entities.insert(player.lock().unwrap().get_id(), Arc::clone(&player));
			}

			let (tx, rx) = channel::<OwnedMessage>();
			thread::spawn(move || {
				let disconnect = || {
					let player = player2.lock().unwrap();
					let mut entities = entities.lock().unwrap();
					entities.remove(&player.get_id());
					println!("\x1B[90m[{}]\x1B[0m {} has disconnected.", Local::now().format("%T"), player.get_name());
				};
				loop {
					if let Ok(message) = rx.recv() {
						if let Err(_error) = sender.send_message(&message) {
							disconnect();
							break;
						}
					} else {
						disconnect();
						break;
					}
				}
			});

			let tx2 = tx.clone();
			thread::spawn(move || {
				loop {
					thread::sleep(Duration::from_millis(16));
					if let Err(_error) = tx2.send(OwnedMessage::Text(Message::Tick.encode())) {
						break;
					}
				}
			});

			for message in receiver.incoming_messages() {
				let player = Arc::clone(&player);
				
				if let Ok(message) = message {
					match message {
						#[allow(unused_must_use)]
						OwnedMessage::Close(_) => {
							tx.send(OwnedMessage::Close(None));
							break;
						}
						OwnedMessage::Ping(ping) => {
							let message = OwnedMessage::Pong(ping);
							if tx.send(message).is_err() { break; };
						}
						OwnedMessage::Text(data) => {
							if let Ok(message) = Message::decode(data) {
								match message {
									Message::InitServer{username, screen_width: _, screen_height: _, password: _} => {
										println!("\x1B[90m[{}]\x1B[0m {} has connected.", Local::now().format("%T"), username);
	
										{
											let mut player = player.lock().unwrap();
											player.set_entity_name(username);
											if tx.send(OwnedMessage::Text(Message::CreateEntity(player.clone()).encode())).is_err() { break; }
											if tx.send(OwnedMessage::Text(Message::InitClient{id: player.get_id()}.encode())).is_err() { break; }
										}
										
	
										let map = map.lock().unwrap();
										for i in 0..8 {
											for j in 0..4 {
												if tx.send(OwnedMessage::Text(Message::Chunk(map.get_chunk(loaded_chunks_top_left.0 + i * 8, loaded_chunks_top_left.1 + j * 8)).encode())).is_err() { break; }
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
	
											// if we must load chunks to left
											if needed_chunks_top_left.0 < loaded_chunks_top_left.0 {
												for i in 0..(loaded_chunks_top_left.0 - needed_chunks_top_left.0) / 8 {
													for j in 0..4 {
														if tx.send(OwnedMessage::Text(Message::Chunk(map.get_chunk(needed_chunks_top_left.0 + i * 8, loaded_chunks_top_left.1 + j * 8)).encode())).is_err() { break; }
														if tx.send(OwnedMessage::Text(Message::UnloadChunk{x: needed_chunks_top_left.0 + i * 8 + 64, y: loaded_chunks_top_left.1 + j * 8}.encode())).is_err() { break; }
													}
												}
												loaded_chunks_top_left.0 = needed_chunks_top_left.0;
											} else if needed_chunks_top_left.0 > loaded_chunks_top_left.0 {
												for i in 0..(needed_chunks_top_left.0 - loaded_chunks_top_left.0) / 8 {
													for j in 0..4 {
														if tx.send(OwnedMessage::Text(Message::Chunk(map.get_chunk(loaded_chunks_top_left.0 + i * 8 + 64, loaded_chunks_top_left.1 + j * 8)).encode())).is_err() { break; }
														if tx.send(OwnedMessage::Text(Message::UnloadChunk{x: loaded_chunks_top_left.0 + i * 8, y: loaded_chunks_top_left.1 + j * 8}.encode())).is_err() { break; }
													}
												}
												loaded_chunks_top_left.0 = needed_chunks_top_left.0;
											}
	
											// if we must load chunks to top
											if needed_chunks_top_left.1 < loaded_chunks_top_left.1 {
												for i in 0..8 {
													for j in 0..(loaded_chunks_top_left.1 - needed_chunks_top_left.1) / 8 {
														if tx.send(OwnedMessage::Text(Message::Chunk(map.get_chunk(loaded_chunks_top_left.0 + i * 8, needed_chunks_top_left.1 + j * 8)).encode())).is_err() { break; }
														if tx.send(OwnedMessage::Text(Message::UnloadChunk{x: loaded_chunks_top_left.0 + i * 8, y: needed_chunks_top_left.1 + j * 8 + 64}.encode())).is_err() { break; }
													}
												}
												loaded_chunks_top_left.1 = needed_chunks_top_left.1;
											} else if needed_chunks_top_left.1 > loaded_chunks_top_left.1 {
												for i in 0..8 {
													for j in 0..(needed_chunks_top_left.1 - loaded_chunks_top_left.1) / 8 {
														if tx.send(OwnedMessage::Text(Message::Chunk(map.get_chunk(loaded_chunks_top_left.0 + i * 8, loaded_chunks_top_left.1 + 64 + j * 8)).encode())).is_err() { break; }
														if tx.send(OwnedMessage::Text(Message::UnloadChunk{x: loaded_chunks_top_left.0 + i * 8, y: loaded_chunks_top_left.1 + j * 8}.encode())).is_err() { break; }
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
				} else {
					println!("\x1B[90m[{}]\x1B[0m {} has disconnected brutally.", Local::now().format("%T"), player.lock().unwrap().get_name());
				}
			}
		});
	}
}
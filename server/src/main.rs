#![allow(dead_code)]
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use core::fmt::Display;
use std::thread::sleep;
use std::thread;
use websocket::sync::Server;
use websocket::OwnedMessage;
use protocol::message::Message;
use protocol::entity::*;
use protocol::map::*;
use std::sync::mpsc::channel;
use protocol::block::Block;
use protocol::block::BlockCode;
use protocol::block::Orientation;
use chrono::Local;
use std::collections::HashMap;
use std::io;
use std::process;
use std::time::{Duration, SystemTime};
use std::collections::hash_map::Entry;
use protocol::coords::*;

struct Client {
	pub id: u64,
	pub sender: Sender<OwnedMessage>,
	pub receiver: Receiver<Message>,
	pub loaded_chunks_top_left: (u64, u64)
}

const CENTER_POINT: u64 = 9_223_372_036_854_775_808;

fn log(message: impl Display) {
	println!("\x1B[90m[{}]\x1B[0m {}", Local::now().format("%T"), message);
}

fn main() {
	let server = match Server::bind("localhost:51034") {
		Ok(server) => server,
		Err(error) if error.kind() == std::io::ErrorKind::AddrInUse => {
			println!("The port 51034 is already in use.");
			
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
						match Server::bind(format!("localhost:51034")) {
							Ok(server) => {
								break server;
							},
							Err(error) if error.kind() == std::io::ErrorKind::AddrInUse => {
								panic!("The port 51034 is still used.");
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

	
	
	let (senders_tx, senders_rx) = channel();
	let (clients_tx, clients_rx) = channel();
	let (commands_tx, commands_rx) = channel();

	thread::spawn(move || {
		loop {
			let mut input = String::new();
			io::stdin().read_line(&mut input).expect("expected stdin stream");
			commands_tx.send(input);
		}
	});

	// accept connections and receive messages
	thread::spawn(move || {
		for request in server.filter_map(Result::ok) {
			if let Ok(client) = request.use_protocol("dungeon_game_protocol").accept() {
				if let Ok((mut third_level_receiver, third_level_sender)) = client.split() {
					//let _ip = client.peer_addr().unwap();
					let (second_level_receiver, first_level_receiver): (Sender<Message>, Receiver<Message>) = channel();
					let (first_level_sender, second_level_sender): (Sender<OwnedMessage>, Receiver<OwnedMessage>) = channel();

					senders_tx.send((second_level_sender, third_level_sender)).expect("the main thread crashed");
					clients_tx.send((first_level_sender, first_level_receiver)).expect("the main thread crashed");
					
					thread::spawn(move || {
						for message in third_level_receiver.incoming_messages() {
							if let Ok(message) = message {
								match message {
									OwnedMessage::Close(_) => {
										log("Client disconnected");
										break;
									}
									OwnedMessage::Text(message) => {
										if let Ok(message) = Message::decode(message) {
											second_level_receiver.send(message).expect("the main thread crashed");
										} else {
											log("can't decode message");
										}
									}
									_ => ()
								}
							} else {
								log("Client was disconnect unproperly");
								break;
							}
						}
					});
				} else {
					log("cannot split client")
				}
			} else {
				log("cannot accept client");
			}
		}
	});

	// send messages
	thread::spawn(move || {
		let mut senders = Vec::new();
		loop {
			while let Ok(sender) = senders_rx.try_recv() {
				senders.push(sender);
			}
			for idx in 0..senders.len() {
				while let Ok(message) = senders[idx].0.try_recv() {
					senders[idx].1.send_message(&message);
				}
			}
			sleep(Duration::from_millis(5));
		}
	});

	let mut map: Map = Map::new();
	let mut entities: HashMap<u64, Entity> = HashMap::new();
	let mut clients: Vec<Client> = Vec::new();

	for x in 0..12 {
		map[(9_223_372_036_854_775_810 + x, 9_223_372_036_854_775_807)] = Block::new(BlockCode::SimpleWall, Orientation::Up);
	}
	for y in 0..8 {
		map[(9_223_372_036_854_775_810, 9_223_372_036_854_775_807 + y)] = Block::new(BlockCode::SimpleWall, Orientation::Up);
		map[(9_223_372_036_854_775_810 + 11, 9_223_372_036_854_775_807 + y)] = Block::new(BlockCode::SimpleWall, Orientation::Up);
	}

	loop {
		while let Ok(client) = clients_rx.try_recv() {
			let entity = Entity::spawn_player("undefined".to_string());
			let client = Client {
				id: entity.get_id(),
				sender: client.0,
				receiver: client.1,
				loaded_chunks_top_left: (9_223_372_036_854_775_808 - 4 * 8, 9_223_372_036_854_775_808 - 2 * 8),
			};
			clients.push(client);
			entities.insert(entity.get_id(), entity);
		}

		for idx in 0..clients.len() {
			let player = entities.get_mut(&clients[idx].id).expect("entity should be existing");

			while let Ok(message) = clients[idx].receiver.try_recv() {
				match message {
					Message::InitServer{username, screen_width: _, screen_height: _, password: _} => {
						log(format!("{} has connected", username));
						
						player.set_entity_name(username);
						if clients[idx].sender.send(OwnedMessage::Text(Message::CreateEntity(player.clone()).encode())).is_err() { break; }
						if clients[idx].sender.send(OwnedMessage::Text(Message::InitClient{id: player.get_id()}.encode())).is_err() { break; }
						
						for i in 0..8 {
							for j in 0..4 {
								if clients[idx].sender.send(OwnedMessage::Text(Message::Chunk(map.get_chunk(clients[idx].loaded_chunks_top_left.0 + i * 8, clients[idx].loaded_chunks_top_left.1 + j * 8)).encode())).is_err() { break; }
							}
						}
					},
					Message::TpEntity{id, coords} => {
						if id == clients[idx].id {
							if !map[coords.clone().into()].is_solid() && !map[(coords.clone() + Coords::new(SingleAxis::new(1, 0), SingleAxis::new(0, 0))).into()].is_solid() && !map[(coords.clone() - Coords::new(SingleAxis::new(0, 0), SingleAxis::new(1, 0))).into()].is_solid()  && !map[(coords.clone() + Coords::new(SingleAxis::new(1, 0), SingleAxis::new(0, 0)) - Coords::new(SingleAxis::new(0, 0), SingleAxis::new(1, 0))).into()].is_solid() && player.coords.distance_from(&coords) <= player.get_speed().into() {
								player.coords = coords;
							} else {
								if clients[idx].sender.send(OwnedMessage::Text(Message::TpEntity{id: player.get_id(), coords: player.coords.clone()}.encode())).is_err() { break; };
							}
							let player_chunk_coords = (player.coords.x.main - (player.coords.x.main % 8), player.coords.y.main - (player.coords.y.main % 8));
							let needed_chunks_top_left = (player_chunk_coords.0 - 4*8, player_chunk_coords.1 - 2*8);

							// if we must load chunks to left
							if needed_chunks_top_left.0 < clients[idx].loaded_chunks_top_left.0 {
								for i in 0..(clients[idx].loaded_chunks_top_left.0 - needed_chunks_top_left.0) / 8 {
									for j in 0..4 {
										if clients[idx].sender.send(OwnedMessage::Text(Message::Chunk(map.get_chunk(needed_chunks_top_left.0 + i * 8, clients[idx].loaded_chunks_top_left.1 + j * 8)).encode())).is_err() { break; }
										if clients[idx].sender.send(OwnedMessage::Text(Message::UnloadChunk{x: needed_chunks_top_left.0 + i * 8 + 64, y: clients[idx].loaded_chunks_top_left.1 + j * 8}.encode())).is_err() { break; }
									}
								}
								clients[idx].loaded_chunks_top_left.0 = needed_chunks_top_left.0;
							} else if needed_chunks_top_left.0 > clients[idx].loaded_chunks_top_left.0 {
								for i in 0..(needed_chunks_top_left.0 - clients[idx].loaded_chunks_top_left.0) / 8 {
									for j in 0..4 {
										if clients[idx].sender.send(OwnedMessage::Text(Message::Chunk(map.get_chunk(clients[idx].loaded_chunks_top_left.0 + i * 8 + 64, clients[idx].loaded_chunks_top_left.1 + j * 8)).encode())).is_err() { break; }
										if clients[idx].sender.send(OwnedMessage::Text(Message::UnloadChunk{x: clients[idx].loaded_chunks_top_left.0 + i * 8, y: clients[idx].loaded_chunks_top_left.1 + j * 8}.encode())).is_err() { break; }
									}
								}
								clients[idx].loaded_chunks_top_left.0 = needed_chunks_top_left.0;
							}

							// if we must load chunks to top
							if needed_chunks_top_left.1 < clients[idx].loaded_chunks_top_left.1 {
								for i in 0..8 {
									for j in 0..(clients[idx].loaded_chunks_top_left.1 - needed_chunks_top_left.1) / 8 {
										if clients[idx].sender.send(OwnedMessage::Text(Message::Chunk(map.get_chunk(clients[idx].loaded_chunks_top_left.0 + i * 8, needed_chunks_top_left.1 + j * 8)).encode())).is_err() { break; }
										if clients[idx].sender.send(OwnedMessage::Text(Message::UnloadChunk{x: clients[idx].loaded_chunks_top_left.0 + i * 8, y: needed_chunks_top_left.1 + j * 8 + 64}.encode())).is_err() { break; }
									}
								}
								clients[idx].loaded_chunks_top_left.1 = needed_chunks_top_left.1;
							} else if needed_chunks_top_left.1 > clients[idx].loaded_chunks_top_left.1 {
								for i in 0..8 {
									for j in 0..(needed_chunks_top_left.1 - clients[idx].loaded_chunks_top_left.1) / 8 {
										if clients[idx].sender.send(OwnedMessage::Text(Message::Chunk(map.get_chunk(clients[idx].loaded_chunks_top_left.0 + i * 8, clients[idx].loaded_chunks_top_left.1 + 64 + j * 8)).encode())).is_err() { break; }
										if clients[idx].sender.send(OwnedMessage::Text(Message::UnloadChunk{x: clients[idx].loaded_chunks_top_left.0 + i * 8, y: clients[idx].loaded_chunks_top_left.1 + j * 8}.encode())).is_err() { break; }
									}
								}
								clients[idx].loaded_chunks_top_left.1 = needed_chunks_top_left.1;
							}
						} else {
							println!("attempt to move an unowned entity");
						}
					},
					/*Message::Ping => {
						if waiting_ping == None {
							if clients[idx].sender.send(OwnedMessage::Text(Message::Ping.encode())).is_err() { break; }
						} else {
							let waiting_ping = waiting_ping.unwrap();
							match  waiting_ping.elapsed() {
								Ok(elapsed) => log(format!("ping: {}ms", elapsed.as_millis())),
								Err(e) => eprintln!("error while getting ping: {}", e)
							}
						}
					},*/
					message => {
						println!("{:?}", message);
					},
				}
			}

			if clients[idx].sender.send(OwnedMessage::Text(Message::Tick.encode())).is_err() { break; }
		}

		while let Ok(command) = commands_rx.try_recv() {
			let words: Vec<&str> = command.trim().split(' ').collect();
			if words.len() > 0 {
				match words[0] {
					"help" => println!("COMMANDS LIST:\n\
						- help => display this page\n\
						- tp [id] [x] [y] => teleport an entity where you want\n\
						- list_entities players => list the connected players"),
					"tp" => {
						if words.len() != 4 {
							println!("tp command require 3 arguments");
						} else {
							if let Ok(id) = words[1].parse::<u64>() {
								if let Ok(x) = words[2].parse::<u64>() {
									if let Ok(y) = words[3].parse::<u64>() {
										if let Entry::Occupied(mut entity) = entities.entry(id) {
											let mut entity = entity.get_mut();
											entity.coords.x.main = x;
											entity.coords.y.main = y;
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
							
							for (id, entity) in entities.iter() {
								if *entity.get_type() == EntityType::Player {
									connected_players.push((*id, entity));
								}
							}

							println!("{} players connected", connected_players.len());

							for (id, entity) in connected_players {
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

		sleep(Duration::from_millis(16));
	}
}
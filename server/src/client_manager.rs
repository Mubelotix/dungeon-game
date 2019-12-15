use std::sync::mpsc::SendError;
use std::sync::mpsc::RecvError;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::time::SystemTime;
use protocol::map::Map;
use protocol::entity::Entity;
use std::collections::HashMap;
use websocket::OwnedMessage;
use std::time::Duration;
use protocol::message::Message;
use crate::log;

pub struct Client {
    map: Arc<Mutex<Map>>,
    entities: Arc<Mutex<HashMap<u64, Arc<Mutex<Entity>>>>>,
    entity: Arc<Mutex<Entity>>
}

impl Client {
    pub fn new(request: websocket::server::upgrade::WsUpgrade<std::net::TcpStream, std::option::Option<websocket::server::upgrade::sync::Buffer>>, map: Arc<Mutex<Map>>, entities: Arc<Mutex<HashMap<u64, Arc<Mutex<Entity>>>>>) -> Self {
        let client = request.use_protocol("dungeon_game_protocol").accept().unwrap();
        //let _ip = client.peer_addr().unwrap();
        let (websocket_receiver, websocket_sender) = client.split().unwrap();

        

        let entity = Arc::new(Mutex::new(Entity::spawn_player("undefined".to_string())));
        
        {
            let mut entities = match entities.lock() {
                Ok(entities) => entities,
                Err(poisoned) => poisoned.into_inner()
            };
            let player = entity.lock().unwrap();
            entities.insert(player.get_id(), Arc::clone(&entity));
        }

        let client = Self {
            map,
            entities,
            entity
        };

        client.launch(websocket_sender, websocket_receiver);

        client
    }

    fn launch(&self, mut sender: websocket::sender::Writer<std::net::TcpStream>, mut receiver: websocket::receiver::Reader<std::net::TcpStream>) {
        let entity = Arc::clone(&self.entity);
        let entity2 = Arc::clone(&self.entity);
        let entities = Arc::clone(&self.entities);
        let map = Arc::clone(&self.map);
        let player_id = entity.lock().unwrap().get_id();
        
        let (websocket, sender_receiver) = channel::<OwnedMessage>();
        let (receiver_sender, receiver_receiver) = channel::<Message>();

        // envoyer les messages
        thread::spawn(move || {
            let disconnect = || {
                let entity = match entity.lock() {
                    Ok(entity) => entity,
                    Err(poisoned) => poisoned.into_inner()
                };
                let mut entities = match entities.lock() {
                    Ok(entities) => entities,
                    Err(poisoned) => poisoned.into_inner()
                };
                entities.remove(&entity.get_id());
                log(format!("{} has disconnected", entity.get_name()));
            };
            
            loop {
                if let Ok(message) = sender_receiver.recv() {
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

        // envoyer les tick 
        let websocket2 = websocket.clone();
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(16));
                if let Err(_error) = websocket2.send(OwnedMessage::Text(Message::Tick.encode())) {
                    break;
                }
            }
        });

        
        thread::spawn(move || {
            let mut loaded_chunks_top_left: (u64, u64) = (9_223_372_036_854_775_808 - 4 * 8, 9_223_372_036_854_775_808 - 2 * 8);
            let mut waiting_ping: Option<SystemTime> = None;

            for message in receiver.incoming_messages() {
				let player = Arc::clone(&entity2);
				
				if let Ok(message) = message {
					match message {
						#[allow(unused_must_use)]
						OwnedMessage::Close(_) => {
							websocket.send(OwnedMessage::Close(None));
							break;
						}
						OwnedMessage::Ping(ping) => {
							let message = OwnedMessage::Pong(ping);
							if websocket.send(message).is_err() { break; };
						}
						OwnedMessage::Text(data) => {
							if let Ok(message) = Message::decode(data) {
								match message {
									Message::InitServer{username, screen_width: _, screen_height: _, password: _} => {
										log(format!("{} has connected", username));
	
										{
											let mut player = match player.lock() {
												Ok(player) => player,
												Err(poisoned) => poisoned.into_inner()
											};
											player.set_entity_name(username);
											if websocket.send(OwnedMessage::Text(Message::CreateEntity(player.clone()).encode())).is_err() { break; }
											if websocket.send(OwnedMessage::Text(Message::InitClient{id: player.get_id()}.encode())).is_err() { break; }
										}
										
	
										let map = match map.lock() {
											Ok(map) => map,
											Err(poisoned) => poisoned.into_inner()
										};
										for i in 0..8 {
											for j in 0..4 {
												if websocket.send(OwnedMessage::Text(Message::Chunk(map.get_chunk(loaded_chunks_top_left.0 + i * 8, loaded_chunks_top_left.1 + j * 8)).encode())).is_err() { break; }
											}
										}
									},
									Message::TpEntity{id, coords} => {
										if id == player_id {
											let map = match map.lock() {
												Ok(map) => map,
												Err(poisoned) => poisoned.into_inner()
											};
											let mut player = match player.lock() {
												Ok(player) => player,
												Err(poisoned) => poisoned.into_inner()
											};
											if !map[coords.clone().into()].is_solid() && player.coords.distance_from(&coords) <= player.get_speed().into() {
												player.coords = coords;
											} else {
												if websocket.send(OwnedMessage::Text(Message::TpEntity{id: player.get_id(), coords: player.coords.clone()}.encode())).is_err() { break; };
											}
											let player_chunk_coords = (player.coords.x.main - (player.coords.x.main % 8), player.coords.y.main - (player.coords.y.main % 8));
											let needed_chunks_top_left = (player_chunk_coords.0 - 4*8, player_chunk_coords.1 - 2*8);
	
											// if we must load chunks to left
											if needed_chunks_top_left.0 < loaded_chunks_top_left.0 {
												for i in 0..(loaded_chunks_top_left.0 - needed_chunks_top_left.0) / 8 {
													for j in 0..4 {
														if websocket.send(OwnedMessage::Text(Message::Chunk(map.get_chunk(needed_chunks_top_left.0 + i * 8, loaded_chunks_top_left.1 + j * 8)).encode())).is_err() { break; }
														if websocket.send(OwnedMessage::Text(Message::UnloadChunk{x: needed_chunks_top_left.0 + i * 8 + 64, y: loaded_chunks_top_left.1 + j * 8}.encode())).is_err() { break; }
													}
												}
												loaded_chunks_top_left.0 = needed_chunks_top_left.0;
											} else if needed_chunks_top_left.0 > loaded_chunks_top_left.0 {
												for i in 0..(needed_chunks_top_left.0 - loaded_chunks_top_left.0) / 8 {
													for j in 0..4 {
														if websocket.send(OwnedMessage::Text(Message::Chunk(map.get_chunk(loaded_chunks_top_left.0 + i * 8 + 64, loaded_chunks_top_left.1 + j * 8)).encode())).is_err() { break; }
														if websocket.send(OwnedMessage::Text(Message::UnloadChunk{x: loaded_chunks_top_left.0 + i * 8, y: loaded_chunks_top_left.1 + j * 8}.encode())).is_err() { break; }
													}
												}
												loaded_chunks_top_left.0 = needed_chunks_top_left.0;
											}
	
											// if we must load chunks to top
											if needed_chunks_top_left.1 < loaded_chunks_top_left.1 {
												for i in 0..8 {
													for j in 0..(loaded_chunks_top_left.1 - needed_chunks_top_left.1) / 8 {
														if websocket.send(OwnedMessage::Text(Message::Chunk(map.get_chunk(loaded_chunks_top_left.0 + i * 8, needed_chunks_top_left.1 + j * 8)).encode())).is_err() { break; }
														if websocket.send(OwnedMessage::Text(Message::UnloadChunk{x: loaded_chunks_top_left.0 + i * 8, y: needed_chunks_top_left.1 + j * 8 + 64}.encode())).is_err() { break; }
													}
												}
												loaded_chunks_top_left.1 = needed_chunks_top_left.1;
											} else if needed_chunks_top_left.1 > loaded_chunks_top_left.1 {
												for i in 0..8 {
													for j in 0..(needed_chunks_top_left.1 - loaded_chunks_top_left.1) / 8 {
														if websocket.send(OwnedMessage::Text(Message::Chunk(map.get_chunk(loaded_chunks_top_left.0 + i * 8, loaded_chunks_top_left.1 + 64 + j * 8)).encode())).is_err() { break; }
														if websocket.send(OwnedMessage::Text(Message::UnloadChunk{x: loaded_chunks_top_left.0 + i * 8, y: loaded_chunks_top_left.1 + j * 8}.encode())).is_err() { break; }
													}
												}
												loaded_chunks_top_left.1 = needed_chunks_top_left.1;
											}
										} else {
											println!("attempt to move an unowned entity2");
										}
									},
									Message::Ping => {
										if waiting_ping == None {
											if websocket.send(OwnedMessage::Text(Message::Ping.encode())).is_err() { break; }
										} else {
											let waiting_ping = waiting_ping.unwrap();
											match  waiting_ping.elapsed() {
												Ok(elapsed) => log(format!("ping: {}ms", elapsed.as_millis())),
												Err(e) => eprintln!("error while getting ping: {}", e)
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
				} else {
					let player = match player.lock() {
						Ok(player) => player,
						Err(poisoned) => poisoned.into_inner()
					};
					log(format!("{} has disconnected brutally", player.get_name()));
				}
			}
        });
    }
}
use wasm_game_lib::{graphics::canvas::*, graphics::image::*, graphics::sprite::*, system::util::*, events::keyboard::{KeyboardManager, Key}};
use wasm_bindgen::{prelude::*, JsCast};
use protocol::message::Message;
use protocol::entity::*;
use protocol::block::*;
use web_sys::{WebSocket, Event, MessageEvent};
use std::rc::Rc;
use protocol::map::Map;
use std::panic;
use std::collections::HashMap;
use console_error_panic_hook;

const CENTER_POINT: u64 = 9_223_372_036_854_775_808;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}
macro_rules! println {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

fn main(mut images: Vec<Image>, websocket: Rc<WebSocket>) {
    println!("Game is ready!");

    let mut canvas = Canvas::new();
    let keyboard = KeyboardManager::new();
    let mut entities: HashMap<u64, Entity> = HashMap::new();
    let mut player_id: u64 = 0;
    let mut map: Map = Map::new();
    let websocket2 = Rc::clone(&websocket);
    
    for image in &mut images {
        image.set_origin((0.0, image.get_size().1 as f64));
    }
    images[2].set_origin((0.0, 0.0));

    websocket.send_with_str(&Message::Init{username: String::from("Mubelotix"), screen_width: canvas.get_size().0, screen_height: canvas.get_size().1, password: None}.encode()).expect("can't send init message");
    let message = Closure::wrap(Box::new(move |event: MessageEvent| {
        if let Some(data) = event.data().as_string() {
            match Message::decode(data).expect("can't deserialize message") {
                Message::ChatMessage{sender_id: _, receiver_id: _, message} => {
                    println!("{}", message);
                },
                Message::Chunk(chunk) => {
                    map.set_chunk(chunk.x, chunk.y, chunk.blocks);
                },
                Message::CreateEntity(entity) => {
                    if entity.get_entity_type() == EntityType::You {
                        player_id = entity.get_id();
                    }
                    entities.insert(entity.get_id(), entity);
                },
                Message::Tick => {
                    if player_id != 0 {
                        let player = &mut entities.get_mut(&player_id).unwrap();

                        if keyboard.get_key(Key::Q) {
                            websocket.send_with_str(&Message::MoveEntity{id: player.get_id(), direction: Orientation::Left}.encode()).unwrap();
                            player.move_in_direction(Orientation::Left);
                        } else if keyboard.get_key(Key::D) {
                            websocket.send_with_str(&Message::MoveEntity{id: player.get_id(), direction: Orientation::Right}.encode()).unwrap();
                            player.move_in_direction(Orientation::Right);
                        } else if keyboard.get_key(Key::Z) {
                            websocket.send_with_str(&Message::MoveEntity{id: player.get_id(), direction: Orientation::Up}.encode()).unwrap();
                            player.move_in_direction(Orientation::Up);
                        } else if keyboard.get_key(Key::S) {
                            websocket.send_with_str(&Message::MoveEntity{id: player.get_id(), direction: Orientation::Down}.encode()).unwrap();
                            player.move_in_direction(Orientation::Down);
                        }
                        
                        canvas.clear();
                        let (x1, y1) = player.get_coords();
                        println!("{:?}", player.get_readable_coords());
                        
                        let x = -25 * 40 + (canvas.get_size().0 / 2) as isize - player.get_position_in_block().0 as isize;
                        let y = -15 as isize * 40 + (canvas.get_size().1 / 2) as isize - player.get_position_in_block().1 as isize;
                            
                            for i in 0..50 {
                                for j in 0..30 {
                                    let i: isize = i - 25;
                                    let j: isize = j - 15;

                                    match map[(x1 + i as u64, y1 + j as u64)].get_block_code() {
                                        BlockCode::SimpleSlab => {
                                            canvas.draw_image((x + (i as isize + 25) * 40) as f64, (y + (j as isize + 15) * 40) as f64 + 80.0, &images[0]);
                                        },
                                        BlockCode::SimpleWall => {
                                            canvas.draw_image((x + (i as isize + 25) * 40) as f64, (y + (j as isize + 15) * 40) as f64 + 80.0, &images[1])
                                        }
                                    }
                                }
                            }

                        canvas.draw_image_with_size(((canvas.get_size().0) / 2) as f64, ((canvas.get_size().1) / 2) as f64, 40.0, 40.0, &images[2])
                    }
                },
                Message::UnloadChunk{x, y} => {
                    map.delete_chunk(x, y);
                },
                Message::Init{username: _, screen_width: _, screen_height: _, password: _} => {
                    panic!("server is not intented to connect");
                },
                Message::MoveEntity{id, direction} => {
                    entities.entry(id).or_default().move_in_direction(direction);
                },
                Message::TpEntity{id, x, y, x2, y2} => {
                    entities.entry(id).or_default().set_position((x, y), (x2, y2));
                }
            };
        } else {
            println!("can't read message as string");
        }
    }) as Box<dyn FnMut(MessageEvent)>);
    websocket2
        .add_event_listener_with_callback("message", message.as_ref().unchecked_ref())
        .unwrap();
    message.forget();


}

fn setup_websocket(images: Vec<Image>) {
    println!("Connecting...");
    let websocket = Rc::new(WebSocket::new_with_str("ws://localhost:2794", "dungeon_game_protocol").unwrap());

    // TODO clear this shit
    let websocket2 = Rc::clone(&websocket);
    let open = Closure::wrap(Box::new(move |_event: Event| {
        let images: Vec<Image> = (&images).clone();
        let websocket = Rc::clone(&websocket2);
        main(images, websocket);
    }) as Box<dyn FnMut(Event)>);
    websocket
        .add_event_listener_with_callback("open", open.as_ref().unchecked_ref())
        .unwrap();
    open.forget();
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    println!("Loading textures...");
    Image::load_images(vec![
        "https://mubelotix.dev/dungeon_game/textures/simple_slab.jpg",
        "https://mubelotix.dev/dungeon_game/textures/simple_wall.jpg",
        "https://mubelotix.dev/dungeon_game/textures/character.png"], setup_websocket);

    Ok(())
}
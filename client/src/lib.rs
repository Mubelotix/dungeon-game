use wasm_game_lib::{graphics::canvas::*, graphics::image::*, graphics::sprite::*, system::util::*, events::keyboard::{KeyboardManager, Key}};
use wasm_bindgen::{prelude::*, JsCast};
use protocol::message::Message;
use protocol::entity::*;
use protocol::block::*;
use web_sys::{WebSocket, Event, MessageEvent};
use std::rc::Rc;
use std::sync::mpsc::channel;
use std::panic;
use console_error_panic_hook;

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

fn main(images: Vec<Image>, websocket: Rc<WebSocket>) {
    println!("Game is ready!");

    let mut canvas = Canvas::new();
    let keyboard = KeyboardManager::new();
    let mut player: Option<Entity> = None;
    let mut chunks: Vec<Chunk> = Vec::new();
    let websocket2 = Rc::clone(&websocket);

    websocket.send_with_str(&Message::Init{username: String::from("Mubelotix"), screen_width: canvas.get_size().0, screen_height: canvas.get_size().1, password: None}.encode()).expect("can't send init message");
    let message = Closure::wrap(Box::new(move |event: MessageEvent| {
        if let Some(data) = event.data().as_string() {
            match Message::decode(data).expect("can't deserialize message") {
                Message::ChatMessage{sender_id: _, receiver_id: _, message} => {
                    println!("{}", message);
                },
                Message::Chunk(chunk) => {
                    chunks.push(chunk);
                },
                Message::CreateEntity(entity) => {
                    if entity.get_entity_type() == EntityType::You {
                        player = Some(entity);
                    }
                },
                Message::Tick => {
                    if let Some(player) = &mut player {
                        if keyboard.get_key(Key::Q) {
                            websocket.send_with_str(&Message::MoveEntity{id: player.get_id(), lenght: player.get_speed(), direction: Orientation::Left}.encode()).unwrap();
                            player.move_in_direction(Orientation::Left);
                        } else if keyboard.get_key(Key::D) {
                            websocket.send_with_str(&Message::MoveEntity{id: player.get_id(), lenght: player.get_speed(), direction: Orientation::Right}.encode()).unwrap();
                            player.move_in_direction(Orientation::Right);
                        } else if keyboard.get_key(Key::Z) {
                            websocket.send_with_str(&Message::MoveEntity{id: player.get_id(), lenght: player.get_speed(), direction: Orientation::Up}.encode()).unwrap();
                            player.move_in_direction(Orientation::Up);
                        } else if keyboard.get_key(Key::S) {
                            websocket.send_with_str(&Message::MoveEntity{id: player.get_id(), lenght: player.get_speed(), direction: Orientation::Down}.encode()).unwrap();
                            player.move_in_direction(Orientation::Down);
                        }
                        
                        canvas.clear();
                        let (x, y) = player.get_coords();
                        println!("{} {}", x, y);
                        for chunk in &chunks {
                            let x = (chunk.x - x) as isize * 40 + (canvas.get_size().0 / 2) as isize - player.get_position_in_block().0 as isize;
                            let y = (chunk.y - y) as isize * 40 + (canvas.get_size().1 / 2) as isize - player.get_position_in_block().1 as isize;
                            
                            for i in 0..8 {
                                for j in 0..8 {
                                    match chunk.blocks[i][j].get_block_code() {
                                        BlockCode::SimpleSlab => {
                                            canvas.draw_image_with_size((x + i as isize * 40) as f64, (y + j as isize * 40) as f64, 40.0, 40.0, &images[0]);
                                        },
                                        BlockCode::SimpleWall => {
                                            canvas.draw_image_with_size((x + i as isize * 40) as f64, (y + j as isize * 40) as f64, 40.0, 40.0, &images[1])
                                        }
                                    }
                                }
                            }
                        }

                        canvas.draw_image_with_size(((canvas.get_size().0) / 2) as f64, ((canvas.get_size().1) / 2) as f64, 40.0, 40.0, &images[2])
                    }
                }
                i => {
                    panic!("server is not intented to send this {:?}", i);
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
    Image::load_images(vec!["https://cdnb.artstation.com/p/assets/images/images/001/146/571/large/ulrick-wery-tileableset2-groundslab.jpg?1441028618", "https://3docean.img.customer.envatousercontent.com/files/45172611/main1_590.JPG?auto=compress%2Cformat&fit=crop&crop=top&w=590&h=590&s=a4ea7fe36777345d561d83dc79e55874", "https://p7.hiclipart.com/preview/924/660/881/2d-computer-graphics-video-game-character-concept-art-2d-game-character-sprites.jpg"], setup_websocket);

    Ok(())
}
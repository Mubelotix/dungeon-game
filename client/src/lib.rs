use wasm_game_lib::{graphics::canvas::*, graphics::image::*, graphics::sprite::*, system::util::*, events::*};
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
    let mut canvas = Canvas::new();
    // TODO Smooth

    // configure the websocket
    let (tx, rx) = channel::<Message>();
    let message = Closure::wrap(Box::new(move |event: MessageEvent| {
        if let Some(data) = event.data().as_string() {
            match Message::decode(data) {
                Ok(message) => if let Err(error) = tx.send(message) {
                    panic!("error while using transmitter: {}", error);
                },
                Err(error) => println!("error decoding message: {}", error),
            };
        } else {
            println!("can't read message as string");
        }
    }) as Box<dyn FnMut(MessageEvent)>);
    websocket
        .add_event_listener_with_callback("message", message.as_ref().unchecked_ref())
        .unwrap();
    message.forget();

    websocket.send_with_str(&Message::Init{username: String::new(), screen_width: canvas.get_size().0, screen_height: canvas.get_size().1, password: None}.encode()).expect("can't send init message");

    let mut player: Option<Entity> = None;
    let mut chunks: Vec<Chunk> = Vec::new();
    
    println!("Game is ready!");
    let main_loop = Closure::wrap(Box::new(move || {
        // inputs
        while let Ok(message) = rx.try_recv() {
            match message {
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
                Message::Init{username: _, password: _, screen_width: _, screen_height: _} => {
                    panic!("server is not intented to send init");
                }
            };
        }

        // draw chunks
        canvas.clear();
        if let Some(player) = &player {
            let (x, y) = player.get_coords();
            for chunk in &chunks {
                
                let x = (chunk.x - x) as usize + (canvas.get_size().0 / 2) as usize;
                let y = (chunk.y - y) as usize + (canvas.get_size().1 / 2) as usize;
                
                for i in 0..8 {
                    for j in 0..8 {
                        match chunk.blocks[i][j].get_block_code() {
                            BlockCode::SimpleSlab => {
                                canvas.draw_image_with_size((x + i * 40) as f64, (y + j * 40) as f64, 40.0, 40.0, &images[0])
                            },
                            BlockCode::SimpleWall => {
                                canvas.draw_image_with_size((x + i * 40) as f64, (y + j * 40) as f64, 40.0, 40.0, &images[1])
                            }
                        }
                    }
                }
            }
        }
        

        
    }) as Box<dyn FnMut()>);

    window()
        .set_interval_with_callback_and_timeout_and_arguments_0(
            main_loop.as_ref().unchecked_ref(),
            16,
        )
        .expect("Can't launch main loop");
    main_loop.forget();
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
    Image::load_images(vec!["https://cdnb.artstation.com/p/assets/images/images/001/146/571/large/ulrick-wery-tileableset2-groundslab.jpg?1441028618", "https://3docean.img.customer.envatousercontent.com/files/45172611/main1_590.JPG?auto=compress%2Cformat&fit=crop&crop=top&w=590&h=590&s=a4ea7fe36777345d561d83dc79e55874"], setup_websocket);

    Ok(())
}
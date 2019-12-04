use wasm_game_lib::{graphics::canvas::*, graphics::image::*, graphics::sprite::*, system::util::*, events::*};
use wasm_bindgen::{prelude::*, JsCast};
use protocol::message::Message;
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

    println!("Game is ready!");
    let mut canvas = Canvas::new();
    let main_loop = Closure::wrap(Box::new(move || {
        while let Ok(message) = rx.try_recv() {
            match message {
                Message::Connect(_) => {
                    if let Err(error) = websocket.send_with_str(&Message::Connect(String::from("test_user")).encode()) {
                        println!("there was an error while trying to connect: {:?}", error);
                    }
                    println!("You are now connected!");
                },
                Message::ChatMessage(from, to, message) => {
                    println!("{} -> {}: {}", from, to, message);
                },
                Message::Chunk(x, y, blocks) => {
                    println!("received a chunk");
                }
            };
        }

        canvas.clear();
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
    Image::load_images(vec!["https://mubelotix.dev/game/dirt.png"], setup_websocket);

    Ok(())
}



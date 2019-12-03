use wasm_game_lib::{graphics::canvas::*, graphics::image::*, graphics::sprite::*, system::util::*, events::*};
use wasm_bindgen::{prelude::*, JsCast};
use protocol::*;
use web_sys::{WebSocket, Event, MessageEvent};
use std::rc::Rc;
use std::sync::mpsc::channel;

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

fn main(images: &Vec<Image>, websocket: &WebSocket) {
    let mut canvas = Canvas::new();

    let (tx, rx) = channel::<Vec<u8>>();
    let message = Closure::wrap(Box::new(move |event: MessageEvent| {
        
        println!("{:?}", Message::from_string(event.data().as_string().unwrap()));
        /*if let blob = event.data().into() {
            println!("{:?}", blob);
        } else {
            println!("invalid message: {:?} because {:?}", event.data(), Blob::new_with_str_sequence(&event.data()));
        }*/
        
        
    }) as Box<dyn FnMut(MessageEvent)>);
    websocket
        .add_event_listener_with_callback("message", message.as_ref().unchecked_ref())
        .unwrap();
    message.forget();

    let main_loop = Closure::wrap(Box::new(move || {
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
    println!("Textures loaded");

    

    println!("connecting");
    let websocket = Rc::new(WebSocket::new_with_str("ws://localhost:2794", "dungeon_game_protocol").unwrap());

    let websocket2 = Rc::clone(&websocket);
    let open = Closure::wrap(Box::new(move |_event: Event| {
        println!("opened");
        websocket2.send_with_str("test!").unwrap();
        main(&images, &websocket2);
    }) as Box<dyn FnMut(Event)>);
    websocket
        .add_event_listener_with_callback("open", open.as_ref().unchecked_ref())
        .unwrap();
    open.forget();

    

    
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    println!("Program started!");

    println!("Loading textures...");
    Image::load_images(vec!["https://mubelotix.dev/game/dirt.png"], setup_websocket);

    Ok(())
}



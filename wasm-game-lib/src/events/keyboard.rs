use crate::system::util::window;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::KeyboardEvent as wsKeyboardEvent;

/// This store all the keys (well, nearly) and their codes.
pub enum Key {
    BackSpace = 8,
    Tab = 9,
    Enter = 13,
    Shift = 16,
    Ctrl = 17,
    Alt = 18,
    Pause = 19,
    CapsLock = 20,
    Escape = 27,
    PageUp = 33,
    PageDown = 34,
    End = 35,
    Home = 36,
    LeftArrow = 37,
    UpArrow = 38,
    RightArrow = 39,
    DownArrow = 40,
    Insert = 45,
    Delete = 46,
    Zero = 48,
    One = 49,
    Two = 50,
    Three = 51,
    Four = 52,
    Five = 53,
    Six = 54,
    Seven = 55,
    Eight = 56,
    Nine = 57,
    A = 65,
    B = 66,
    C = 67,
    D = 68,
    E = 69,
    F = 70,
    G = 71,
    H = 72,
    I = 73,
    J = 74,
    K = 75,
    L = 76,
    M = 77,
    N = 78,
    O = 79,
    P = 80,
    Q = 81,
    R = 82,
    S = 83,
    T = 84,
    U = 85,
    V = 86,
    W = 87,
    X = 88,
    Y = 89,
    Z = 90,
    LeftWindowKey = 91,
    RightWindowKey = 92,
    SelectKey = 93,
    Numpad0 = 96,
    Numpad1 = 97,
    Numpad2 = 98,
    Numpad3 = 99,
    Numpad4 = 100,
    Numpad5 = 101,
    Numpad6 = 102,
    Numpad7 = 103,
    Numpad8 = 104,
    Numpad9 = 105,
    Multiply = 106,
    Add = 107,
    Subtract = 109,
    DecimalPoint = 110,
    Divide = 111,
    F1 = 112,
    F2 = 113,
    F3 = 114,
    F4 = 115,
    F5 = 116,
    F6 = 117,
    F7 = 118,
    F8 = 119,
    F9 = 120,
    F10 = 121,
    F11 = 122,
    F12 = 123,
    NumLock = 144,
    ScrollLock = 145,
    SemiColon = 186,
    EqualSign = 187,
    Comma = 188,
    Dash = 189,
    Period = 190,
    ForwardSlash = 191,
    GraveAccent = 192,
    OpenBracket = 219,
    BackSlash = 220,
    CloseBraket = 221,
    SingleQuote = 222,
}

/// This struct contains all the states of the keys and update them automatically.
/// For now you must not let this struct be dropped!
pub struct KeyboardManager {
    keys: Rc<RefCell<[bool; 256]>>,
}

impl Default for KeyboardManager {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyboardManager {
    /// Create a keyboard manager.
    /// The stated of keys stored in the struct will be updtated.
    pub fn new() -> Self {
        let mut manager = KeyboardManager {
            keys: Rc::new(RefCell::new([false; 256])),
        };
        manager.launch();
        manager
    }

    /// Return the state of the key.
    /// True if pressed, false otherwise.
    pub fn get_key(&self, key_code: Key) -> bool {
        let key = Rc::clone(&self.keys);
        let key = key.borrow();
        key[key_code as usize]
    }

    /// Launch the updtates.
    fn launch(&mut self) {
        let keys = Rc::clone(&self.keys);
        let keys2 = Rc::clone(&self.keys);

        let keydown = Closure::wrap(Box::new(move |event: wsKeyboardEvent| {
            //console::log_1(&format!("keydown event: keycode = {}", event.key_code()).into());

            let mut key = keys.borrow_mut();
            key[event.key_code() as usize] = true;
        }) as Box<dyn FnMut(wsKeyboardEvent)>);
        window()
            .add_event_listener_with_callback("keydown", keydown.as_ref().unchecked_ref())
            .unwrap();
        keydown.forget();

        /*let keypress = Closure::wrap(Box::new(move |_event: wsKeyboardEvent| {
            //console::log_1(&format!("keypress event: keycode = {}", event.key_code()).into());
            // TODO use this event ?
        }) as Box<dyn FnMut(wsKeyboardEvent)>);
        window()
            .add_event_listener_with_callback("keypress", keypress.as_ref().unchecked_ref())
            .unwrap();
        keypress.forget();*/

        let keyup = Closure::wrap(Box::new(move |event: wsKeyboardEvent| {
            //console::log_1(&format!("keyup event: keycode = {}", event.key_code()).into());

            let mut key = keys2.borrow_mut();
            key[event.key_code() as usize] = false;
        }) as Box<dyn FnMut(wsKeyboardEvent)>);
        window()
            .add_event_listener_with_callback("keyup", keyup.as_ref().unchecked_ref())
            .unwrap();
        keyup.forget();
    }
}

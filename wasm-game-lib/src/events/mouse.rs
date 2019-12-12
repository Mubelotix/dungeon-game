use crate::system::util::window;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::MouseEvent;

/// This struct contains the state of the mouse (position and click)
/// For now you must not let this struct be dropped!
pub struct Mouse {
    pressed: bool,
    position: Rc<RefCell<(i32, i32)>>,
}

impl Default for Mouse {
    fn default() -> Self {
        Self::new()
    }
}

impl Mouse {
    /// Create a mouse interface.
    /// The stated of keys stored in the struct will be updtated.
    pub fn new() -> Self {
        let mut mouse = Mouse {
            pressed: false,
            position: Rc::new(RefCell::new((0, 0))),
        };
        mouse.launch();
        mouse
    }

    /// Return the position of the mouse.
    pub fn get_position(&self) -> (i32, i32) {
        *Rc::clone(&self.position).borrow()
    }

    /// Launch the updates.
    fn launch(&mut self) {
        let position = Rc::clone(&self.position);

        let mousemove = Closure::wrap(Box::new(move |event: MouseEvent| {
            let mut position = position.borrow_mut();
            *position = (event.client_x(), event.client_x());
        }) as Box<dyn FnMut(MouseEvent)>);
        window()
            .add_event_listener_with_callback("mousemove", mousemove.as_ref().unchecked_ref())
            .unwrap();
        mousemove.forget();
    }
}

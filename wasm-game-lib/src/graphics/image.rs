use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Event, HtmlImageElement};

#[derive(Clone)]
pub struct Image {
    html_element: web_sys::HtmlImageElement,
    origin: (f64, f64),
}

impl Image {
    pub fn new(url: &str, closure: &js_sys::Function) -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        let html_element = document
            .create_element("img")
            .unwrap()
            .dyn_into::<web_sys::HtmlImageElement>()
            .unwrap();
        html_element.set_src(url);

        html_element
            .add_event_listener_with_callback("load", closure)
            .unwrap();

        Image { html_element, origin: (0.0, 0.0) }
    }

    // The origin is a tuple who will be added to coordinates everywhere they are used.
    /// It is useful when you want the coordinates of the sprite at the center of the texture and not at the upper left corner.
    pub fn set_origin(&mut self, origin: (f64, f64)) {
        self.origin = origin;
    }

    /// A simple getter for the origin.
    /// See set_origin() method to learn more about what origin is.
    pub fn get_origin(&self) -> (f64, f64) {
        self.origin
    }

    pub fn get_html_element(&self) -> &web_sys::HtmlImageElement {
        &self.html_element
    }

    pub fn get_width(&self) -> u32 {
        self.html_element.width()
    }

    pub fn get_height(&self) -> u32 {
        self.html_element.height()
    }

    pub fn get_size(&self) -> (u32, u32) {
        (self.html_element.width(), self.html_element.height())
    }

    pub fn load_image(url: &str, callback: fn(Image)) {
        let closure = Closure::wrap(Box::new(move |event: Event| {
            let target_event = event.target().unwrap();
            let element_jsvalue = JsValue::from(target_event);
            let html_element = HtmlImageElement::from(element_jsvalue);
            let image = Image { html_element, origin: (0.0, 0.0) };

            callback(image);
        }) as Box<dyn FnMut(Event)>);

        Image::new(&url, closure.as_ref().unchecked_ref());

        closure.forget();
    }

    pub fn load_images(urls: std::vec::Vec<&'static str>, callback: fn(Vec<Image>)) {
        let mut images: Vec<Option<Image>> = Vec::new();
        let total_images = urls.len();
        let mut loaded_images = 0;
        let urls2 = urls.clone(); // TODO supress clone with Rc

        for _ in 0..total_images {
            images.push(None);
        }

        let closure = Closure::wrap(Box::new(move |event: Event| {
            let target_event = event.target().unwrap();
            let element_jsvalue = JsValue::from(target_event);
            let html_element = HtmlImageElement::from(element_jsvalue);

            loaded_images += 1;

            for i in 0..total_images {
                if urls2[i] == html_element.src() && images[i].is_none() {
                    images[i] = Some(Image { html_element, origin: (0.0, 0.0) });
                    break;
                }
            }

            if total_images - loaded_images == 0 {
                let mut images2: Vec<Image> = Vec::new();

                for i in 0..total_images {
                    if let Some(img) = images[i].take() {
                        images2.push(img);
                    } else {
                        panic!("Can't sort images because a url has changed after load ! (maybe a redirection or it was a relative url)");
                    }
                }
                callback(images2.clone());
            }
        }) as Box<dyn FnMut(Event)>);

        for url in urls {
            Image::new(&url, closure.as_ref().unchecked_ref());
        }

        closure.forget();
    }
}

use crate::events::keyboard::KeyboardManager;
use crate::events::mouse::Mouse;
use crate::graphics::image::Image;
use crate::graphics::linecap::LineCap;
use wasm_bindgen::JsCast;

/// Objects who implement this trait can be drawed in a canvas.
/// You can implement this trait manually for your own objects.
pub trait Drawable {
    fn draw(&self, canvas: &mut Canvas);
}

/// The Canvas is where you draw everything.
/// This structs include a [keyboard manager](../../events/keyboard/struct.KeyboardManager.html)
pub struct Canvas {
    canvas_element: web_sys::HtmlCanvasElement,
    canvas_context: web_sys::CanvasRenderingContext2d,
    pub origin: (f64, f64)
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new(false)
    }
}

impl Canvas {
    /// Create a new canvas. Auto adjust the size to the window.
    /// It's using a lot of unwrap for now and will crash if something in the browser don't work.
    pub fn new(visible: bool) -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas_element = document
            .create_element("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        
        if visible {
            document
                .body()
                .unwrap()
                .append_child(&canvas_element)
                .unwrap();
        }

        canvas_element.set_width(document.document_element().unwrap().client_width() as u32);
        canvas_element.set_height(document.document_element().unwrap().client_height() as u32);

        let canvas_context = canvas_element
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        Canvas {
            canvas_element,
            canvas_context,
            origin: (0.0, 0.0),
        }
    }

    /// The origin is a tuple who will be added to coordinates everywhere they are used.
    /// It is useful when you want the coordinates of the sprite at the center of the texture and not at the upper left corner.
    pub fn set_origin(&mut self, origin: (f64, f64)) {
        self.origin = origin;
    }

    /// A simple getter for the origin.
    /// See set_origin() method to learn more about what origin is.
    pub fn get_origin(&self) -> (f64, f64) {
        self.origin
    }

    pub fn fill_rect(&mut self, (x, y): (f64, f64), (w, h): (f64, f64)) {
        self.canvas_context
            .fill_rect(x + self.origin.0, y + self.origin.1, w, h);
    }

    pub fn clear_rect(&mut self, (x, y): (f64, f64), (w, h): (f64, f64)) {
        self.canvas_context
            .clear_rect(x + self.origin.0, y + self.origin.1, w, h);
    }

    /// Clear the canvas
    pub fn clear(&mut self) {
        self.clear_rect(
            (0.0 - self.origin.0, 0.0 - self.origin.1),
            (
                f64::from(self.canvas_element.width()),
                f64::from(self.canvas_element.height()),
            ),
        )
    }

    // TODO stroke rect

    pub fn fill_text(&mut self, (x, y): (f64, f64), text: &str) {
        self.canvas_context
            .fill_text(text, x + self.origin.0, y + self.origin.1)
            .unwrap();
    }

    pub fn stroke_text(&mut self, (x, y): (f64, f64), text: &str) {
        self.canvas_context
            .stroke_text(text, x + self.origin.0, y + self.origin.1)
            .unwrap();
    }

    // TODO measure text

    pub fn get_line_width(&self) -> f64 {
        self.canvas_context.line_width()
    }

    pub fn set_line_width(&mut self, width: f64) {
        self.canvas_context.set_line_width(width);
    }

    pub fn get_line_cap(&self) -> LineCap {
        LineCap::from(&self.canvas_context.line_cap())
    }

    pub fn set_line_cap(&mut self, value: LineCap) {
        self.canvas_context.set_line_cap(value.to_string());
    }

    // TODO other line proporties

    pub fn draw_line(&mut self, (x, y): (f64, f64), (x_dest, y_dest): (f64, f64)) {
        self.canvas_context.begin_path();
        self.canvas_context
            .move_to(x + self.origin.0, y + self.origin.1);
        self.canvas_context
            .line_to(x_dest + self.origin.0, y_dest + self.origin.1);
        self.canvas_context.stroke();
    }

    /// Draw an image
    pub fn draw_image(&mut self, x: f64, y: f64, image: &Image) {
        self.canvas_context
            .draw_image_with_html_image_element(
                image.get_html_element(),
                x + self.origin.0 - image.get_origin().0,
                y + self.origin.1 - image.get_origin().1,
            )
            .unwrap();
    }

    pub fn draw_canvas(&mut self, (x, y): (f64, f64), canvas: &Canvas) {
        self.canvas_context
            .draw_image_with_html_canvas_element(
                &canvas.canvas_element,
                x + self.origin.0 - canvas.get_origin().0,
                y + self.origin.1 - canvas.get_origin().1,
            )
            .unwrap();
    }

    /// Draw a resized image.
    /// Unrecommended due to the loss of performance
    pub fn draw_image_with_size(&mut self, x: f64, y: f64, width: f64, height: f64, image: &Image) {
        self.canvas_context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                image.get_html_element(),
                0.0,
                0.0,
                f64::from(image.get_width()),
                f64::from(image.get_height()),
                x + self.origin.0 - image.get_origin().0,
                y + self.origin.1 - image.get_origin().1,
                width,
                height,
            )
            .unwrap();
    }

    pub fn set_style_property(&mut self, (name, value): (&str, &str)) {
        // TODO unwrap
        self.canvas_element
            .style()
            .set_property(name, value)
            .unwrap();
    }

    /// Set size of canvas.
    /// It does not change the canvas size in the tab.
    /// The point (0,50) in a canvas with a size of 100 is at the same position than the point (0,100) in a canvas who have a size of 200.
    /// Larger the size is, larger is the precision.
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.canvas_element.set_width(width);
        self.canvas_element.set_height(height);
    }

    /// Return the size of the canvas. Unless you use autoresize() or set_size() methods, the canvas size will never change.
    /// If the browser's window is resized, the canvas display surface will be modified so the proportion of the canvas may change.
    /// It means that your program can ignore this and the user will resize to a good proportion or update the tab.
    pub fn get_size(&self) -> (u32, u32) {
        (self.canvas_element.width(), self.canvas_element.height())
    }

    /// Draw an object who implement the Drawable trait. You can create your own Drawable structs and use it here like my structs.
    pub fn draw(&mut self, element: impl Drawable) {
        element.draw(self);
    }
}

impl Drop for Canvas {
    fn drop(&mut self) {
        self.canvas_element.remove();
    }
}
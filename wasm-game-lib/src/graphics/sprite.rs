use crate::graphics::canvas::Canvas;
use crate::graphics::canvas::Drawable;
use crate::graphics::image::Image;

#[derive(Default)]
pub struct Sprite {
    position: (f64, f64),
    origin: (f64, f64),
    size: (f64, f64),
    texture: Option<Image>,
}

impl Sprite {
    pub fn new() -> Self {
        Sprite {
            position: (0.0, 0.0),
            origin: (0.0, 0.0),
            size: (0.0, 0.0),
            texture: None,
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

    /// Set the size
    pub fn set_size(&mut self, width: f64, height: f64) {
        self.size = (width, height);
    }

    /// Return the size
    pub fn get_size(&self) -> (f64, f64) {
        self.size
    }

    /// Set the width
    pub fn set_width(&mut self, width: f64) {
        self.size.0 = width;
    }

    /// Return the width
    pub fn get_width(&self) -> f64 {
        self.size.0
    }

    /// Set the height
    pub fn set_height(&mut self, height: f64) {
        self.size.1 = height;
    }

    /// Return the height
    pub fn get_height(&self) -> f64 {
        self.size.1
    }

    /// Set the width and automatically set the height to keep the proportion.
    /// Return the height.
    pub fn set_width_and_keep_proportion(&mut self, width: f64) -> f64 {
        self.size.1 = self.size.1 / self.size.0 * width;
        self.size.0 = width;
        self.size.1
    }

    /// Set the height and automatically set the width to keep the proportion.
    /// Return the width.
    pub fn set_height_and_keep_proportion(&mut self, height: f64) -> f64 {
        self.size.0 = self.size.0 / self.size.1 * height;
        self.size.1 = height;
        self.size.0
    }

    pub fn set_texture(&mut self, texture: Image) {
        self.size = (
            f64::from(texture.get_width()),
            f64::from(texture.get_height()),
        );
        self.texture = Some(texture);
    }

    pub fn set_position(&mut self, x: f64, y: f64) {
        self.position = (x + self.origin.0, y + self.origin.1);
    }

    pub fn get_position(&self) -> (f64, f64) {
        self.position
    }

    pub fn shift(&mut self, x: f64, y: f64) {
        self.position.0 += x;
        self.position.1 += y;
    }
}

impl Drawable for &Sprite {
    fn draw(&self, canvas: &mut Canvas) {
        canvas.draw_image_with_size(
            self.position.0 - self.origin.0,
            self.position.1 - self.origin.1,
            self.size.0,
            self.size.1,
            &self.texture.as_ref().unwrap(),
        );
    }
}

impl Drawable for Sprite {
    fn draw(&self, canvas: &mut Canvas) {
        canvas.draw_image_with_size(
            self.position.0 - self.origin.0,
            self.position.1 - self.origin.1,
            self.size.0,
            self.size.1,
            &self.texture.as_ref().unwrap(),
        );
    }
}

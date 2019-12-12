//! Wasm is not easy.  
//! Writting a game in wasm is totaly different than writing a native game.  
//! For example, you don't have any sleep function.
//! You must use javascript events for almost everything.  
//! *The goal of this library is to make gamedev on wasm with Rust more simple and more idiomatic.*  
//! Because wasm is the future of games in browsers, you should try!
//! Games on browsers have been small and uninteresting for years.
//! Now we can build great games! With a high quality! Not like old flash games or .io games.

#![warn(clippy::all)]
pub mod events;
pub mod graphics;
pub mod system;

pub use crate::graphics::canvas::Canvas;
pub use crate::graphics::sprite::Sprite;
pub use crate::graphics::image::Image;
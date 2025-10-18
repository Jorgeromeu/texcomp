#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod asset;
mod image;
mod render;
mod selector;
mod viewer;

#[cfg(debug_assertions)]
mod debug;

pub use app::TexCompApp;

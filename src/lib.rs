#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod selector;
mod asset_loader;

pub use app::TexCompApp;
pub use selector::Selector;
pub use asset_loader::load_asset;
pub use asset_loader::AssetType;

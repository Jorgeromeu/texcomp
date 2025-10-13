#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod asset_loader;
mod selector;

pub use app::TexCompApp;
pub use asset_loader::AssetType;
pub use asset_loader::load_asset;
pub use selector::Selector;

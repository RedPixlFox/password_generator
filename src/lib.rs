#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::MyApp;
mod password_generator;
pub use password_generator::{PasswordGenerator, PasswordGeneratorSettings};

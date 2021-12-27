pub mod commands;
pub mod json;
pub mod app;
pub mod config;

fn main() {
    app::handle_matches(config::Config::default());
}

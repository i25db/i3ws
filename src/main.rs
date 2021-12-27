pub mod commands;
pub mod json;
pub mod app;
pub mod config;
pub mod workspace;

fn main() {
    app::handle_matches(config::Config::default());
}

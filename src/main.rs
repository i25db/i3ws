pub(crate) mod app;
pub(crate) mod commands;
pub(crate) mod config;
pub(crate) mod json;
pub(crate) mod workspace;

fn main() {
    app::handle_matches(config::Config::default());
}

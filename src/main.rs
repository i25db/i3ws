mod app;
mod commands;
mod config;
mod workspace;

use config::Config;

fn main() {
    app::handle_matches(Config::get_config_from_file());
}

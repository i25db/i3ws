pub mod commands;
pub mod json;
pub mod app;

fn main() {
    app::handle_matches(app::get_matches());
}

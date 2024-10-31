use app::YewApp;

mod app;
mod socket;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<YewApp>::new().render();
}
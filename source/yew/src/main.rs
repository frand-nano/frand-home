use app::App;

mod app;
mod socket;
mod view;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
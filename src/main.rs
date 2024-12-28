mod app;
mod form;
mod translations;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}

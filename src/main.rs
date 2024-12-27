mod app;
mod form;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}

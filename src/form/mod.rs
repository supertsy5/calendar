
use html::ChildrenProps;
use yew::prelude::*;

pub use checkbox_input::CheckboxInput;
pub use color_input::ColorInput;
pub use int_input::IntInput;
pub use string_input::StringInput;
pub use select::{Select, SelectOption};

pub mod checkbox_input;
pub mod color_input;
pub mod int_input;
pub mod string_input;
pub mod select;

#[function_component]
pub fn Form(props: &ChildrenProps) -> Html {
    html! { <div class="form">
        <table>{ props.children.clone() }</table>
    </div> }
}

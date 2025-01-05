use std::ops::Deref;

use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub name: AttrValue,
    pub checked: bool,
    pub onchange: Callback<bool>,
}

#[function_component]
pub fn CheckboxInput(props: &Props) -> Html {
    let onchange = props.onchange.clone();
    html! { <tr>
        <td>{ props.name.deref() }</td>
        <td>
            <input
                type="checkbox"
                checked={ props.checked }
                onchange={ move |event: Event| {
                    if let Some(element) = event
                        .target()
                        .and_then(|target| target.dyn_into::<HtmlInputElement>().ok())
                    {
                        onchange.emit(element.checked());
                    }
                } }
            />
        </td>
    </tr> }
}

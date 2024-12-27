use std::ops::Deref;

use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub name: AttrValue,
    pub value: AttrValue,
    pub onchange: Callback<String>,
}

#[function_component]
pub fn StringInput(props: &Props) -> Html {
    let onchange = props.onchange.clone();
    html! { <tr>
        <td>{ props.name.deref() }</td>
        <td>
            <input
                type="text"
                value={ props.value.clone() }
                onchange={ move |event: Event| {
                    if let Some(element) = event
                        .target()
                        .and_then(|target| target.dyn_into::<HtmlInputElement>().ok())
                    {
                        onchange.emit(element.value());
                    }
                } }
            />
        </td>
    </tr> }
}
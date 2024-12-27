use std::ops::Deref;

use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub name: AttrValue,
    #[prop_or(AttrValue::Static("text"))]
    pub r#type: AttrValue,
    pub value: AttrValue,
    pub onchange: Callback<String>,
}

#[function_component]
pub fn ColorInput(props: &Props) -> Html {
    let onchange = props.onchange.clone();
    let onchange1 = props.onchange.clone();
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
        <td>
            <input
                type="color"
                value={ props.value.clone() }
                onchange={ move |event: Event| {
                    if let Some(element) = event
                        .target()
                        .and_then(|target| target.dyn_into::<HtmlInputElement>().ok())
                    {
                        onchange1.emit(element.value());
                    }
                } }
            />
        </td>
    </tr> }
}
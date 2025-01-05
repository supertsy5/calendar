use std::ops::Deref;

use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub name: AttrValue,
    pub value: i32,
    #[prop_or_default]
    pub min: Option<i32>,
    #[prop_or_default]
    pub max: Option<i32>,
    pub onchange: Callback<i32>,
}

#[function_component]
pub fn IntInput(props: &Props) -> Html {
    let onchange = props.onchange.clone();
    html! { <tr>
        <td>{ props.name.deref() }</td>
        <td>
            <input
                type="number"
                min={ props.min.as_ref().map_or_else(String::new, i32::to_string) }
                max={ props.max.as_ref().map_or_else(String::new, i32::to_string) }
                value={ props.value.to_string() }
                onchange={ move |event: Event| {
                    if let Some(value) = event
                        .target()
                        .and_then(|target| target.dyn_into::<HtmlInputElement>().ok())
                        .and_then(|element| element.value().parse::<i32>().ok())
                    {
                        onchange.emit(value);
                    }
                } }
            />
        </td>
    </tr> }
}

use std::ops::Deref;

use web_sys::{wasm_bindgen::JsCast, HtmlSelectElement};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub name: AttrValue,
    pub value: u32,
    pub onchange: Callback<u32>,
    pub children: ChildrenWithProps<SelectOption>,
}

#[function_component]
pub fn Select(props: &Props) -> Html {
    let onchange = props.onchange.clone();
    html! { <tr>
        <td>{ props.name.deref() }</td>
        <td>
            <select onchange={ move |event: Event| {
                if let Some(value) = event
                    .target()
                    .and_then(|target| target.dyn_into::<HtmlSelectElement>().ok())
                {
                    onchange.emit(value.selected_index() as u32);
                }
            } }>{
                for props.children
                .iter()
                .enumerate()
                .map(|(i, option)| html! {
                    <option selected={ i == props.value as usize }>
                        { option.props.children.clone() }
                    </option>
                })
            }</select>
        </td>
    </tr> }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct SelectOption {
    pub children: Html,
}

impl Component for SelectOption {
    type Message = ();
    type Properties = SelectOption;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.props().clone()
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {}
    }
}

use yew::prelude::*;
use yew::Properties;

#[derive(PartialEq, Properties)]
pub struct UserListPageItemProps {
    pub onclick: Callback<usize>,
    pub index: usize,
    pub active: bool,
}

#[function_component(PagerItem)]
pub fn pager_item(props: &UserListPageItemProps) -> Html {
    let index = props.index;
    let active = props.active;
    let onclick = {
        let oc = props.onclick.clone();
        Callback::from(move |_| oc.emit(index))
    };
    html! {
        <>
            <li><a href={format!("javascript:void(0)")} {onclick} class={if active {"pagination-link is-current"} else { "pagination-link" }}>{index}</a></li>
        </>
    }
}

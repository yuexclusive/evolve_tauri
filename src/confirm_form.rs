use crate::util::common;
use yew::prelude::*;
use yew::Properties;

#[derive(Clone, PartialEq, Properties)]
pub struct ConfirmFormProps {
    pub onconfirm: Callback<()>,
    pub onclose: Callback<()>,
    pub content: String,
    #[prop_or(true)]
    pub show_cancel: bool,
    #[prop_or(true)]
    pub show_close: bool,
}

#[function_component(ConfirmForm)]
pub fn confirm_form(props: &ConfirmFormProps) -> Html {
    let close = {
        let onclose = props.onclose.clone();
        Callback::from(move |_| {
            onclose.emit(());
        })
    };

    let onconfirm = {
        let onconfirm = props.onconfirm.clone();
        Callback::from(move |_| {
            onconfirm.emit(());
        })
    };

    html! {
        <div class="modal is-active">
            <div class="modal-background"></div>
            <div class="modal-card">
                <header class="modal-card-head">
                <p class="modal-card-title">{"Confirm"}</p>
                <button class="delete" aria-label="close" onclick={close.clone()}></button>
                </header>
                <section class="modal-card-body">
                    {
                        common::create_html("div",&props.content)
                    }
                </section>
                <footer class="modal-card-foot">
                <button class="button is-danger"  onclick={onconfirm}>{"Confirm"}</button>
                <button class="button" onclick={close}>{"Cancel"}</button>
                </footer>
            </div>
        </div>
    }
}

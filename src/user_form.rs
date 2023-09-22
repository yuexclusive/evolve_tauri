use crate::component::message_item::MessageItemValue;
use crate::component::message_list::{self, MessageList};
use crate::util::common;
use user_cli::apis::user_controller_api::{self, UpdateError};
use user_cli::apis::Error;
use user_cli::models::{User, UserUpdateReq};
use wasm_bindgen_futures::spawn_local;
use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;
use yew::Properties;

#[derive(Clone, PartialEq, Properties)]
pub struct UserFormProps {
    #[prop_or_default]
    pub value: User,
    #[prop_or_default]
    pub onupdate: Callback<()>,
    pub onclose: Callback<()>,
}

#[function_component(UserForm)]
pub fn user_form(props: &UserFormProps) -> Html {
    let messages: std::rc::Rc<std::cell::RefCell<Option<MessageItemValue>>> =
        use_mut_ref(|| Default::default());
    let value = use_mut_ref(|| props.value.clone());
    let force_update = use_force_update();

    let close = {
        let onclose = props.onclose.clone();
        Callback::from(move |_e: MouseEvent| {
            onclose.emit(());
        })
    };
    let update = {
        let value = value.clone();
        let messages = messages.clone();
        let onupdate = props.onupdate.clone();
        let force_update = force_update.clone();
        Callback::from(move |_e: MouseEvent| {
            let messages = messages.clone();
            let onupdate = onupdate.clone();
            let force_update = force_update.clone();
            let value = value.clone();
            let value = value.borrow();
            let req = UserUpdateReq {
                id: value.id,
                mobile: value.mobile.clone(),
                name: value.name.clone(),
            };
            spawn_local(async move {
                match user_controller_api::update(&common::get_cli_config().unwrap(), req).await {
                    Ok(_) => {
                        onupdate.emit(());
                    }
                    Err(err) => {
                        match err {
                            Error::ResponseError(res_err) => match res_err.entity {
                                Some(UpdateError::Status400(e))
                                | Some(UpdateError::Status401(e))
                                | Some(UpdateError::Status500(e)) => {
                                    *messages.borrow_mut() =
                                        Some(message_list::error(&format!("{}", e.msg)));
                                }
                                _ => {
                                    *messages.borrow_mut() =
                                        Some(message_list::error(&format!("{}", res_err.content)));
                                }
                            },
                            _ => {
                                *messages.borrow_mut() =
                                    Some(message_list::error(&format!("{}", err)));
                            }
                        }
                        force_update.force_update();
                    }
                };
            });
        })
    };
    let name_change = {
        let value = value.clone();
        Callback::from(move |e: Event| {
            let el: HtmlInputElement = e.target_unchecked_into();
            value.borrow_mut().name = Some(el.value());
        })
    };
    let mobile_change = {
        let value = value.clone();
        Callback::from(move |e: Event| {
            let el: HtmlInputElement = e.target_unchecked_into();
            value.borrow_mut().mobile = Some(el.value());
        })
    };
    let val = value.borrow();
    html! {
        <div class="modal is-active">
            <div class="modal-background"></div>
            <div class="modal-card">
                <MessageList value={messages.borrow().clone()}/>
                <header class="modal-card-head">
                <p class="modal-card-title">{"User Edit"}</p>
                <button class="delete" aria-label="close" onclick={close.clone()}></button>
                </header>
                <section class="modal-card-body">

                <fieldset disabled={true}>
                <div class="field">
                    <label class="label">{"Type"}</label>
                    <div class="control">
                    <input class="input" value={val.r#type.to_string().clone()} type="text" />
                    </div>
                </div>

                <div class="field">
                    <label class="label">{"Email"}</label>
                    <div class="control">
                    <input class="input" value={val.email.clone()} type="email" />
                    </div>
                </div>

                <div class="field">
                    <label class="label">{"Status"}</label>
                    <div class="control">
                    <input class="input" value={val.status.to_string().clone()} />
                    </div>
                </div>
                </fieldset>



                <div class="field">
                    <label class="label">{"Name"}</label>
                    <div class="control">
                    <input class="input" value={val.name.clone()} type="text" placeholder="Scarlett" onchange={name_change}/>
                    </div>
                </div>

                <div class="field">
                    <label class="label">{"Mobile"}</label>
                    <div class="control">
                    <input class="input" value={val.mobile.clone()} type="text" placeholder="13800001111" onchange={mobile_change}/>
                    </div>
                </div>

                </section>
                <footer class="modal-card-foot">
                <button class="button is-success"  onclick={update}>{"Save changes"}</button>
                <button class="button" onclick={close} >{"Cancel"}</button>
                </footer>
            </div>
        </div>
    }
}

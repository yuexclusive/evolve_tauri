use crate::component::message_list::{self, MessageList};
use crate::component::pager::{self, Page, Pager};
use crate::confirm_form::ConfirmForm;
use crate::user_form::UserForm;

use crate::util::common;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use user_cli::apis::user_controller_api;
use user_cli::models::User;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Serialize)]
pub struct DeleteReq {
    pub ids: Vec<i64>,
}

#[function_component(UserList)]
pub fn user_list() -> Html {
    let refresh = use_state(|| false);
    let force_update = use_force_update();
    let selected_row: Rc<RefCell<Option<User>>> = use_mut_ref(|| None);
    let message = use_mut_ref(|| None);
    let key_word = use_mut_ref(|| String::default());
    let user_form_closed = use_mut_ref(|| true);
    let confirm_form_closed = use_mut_ref(|| true);
    let loading = use_mut_ref(|| false);
    let index = use_mut_ref(|| 1);
    let total = use_mut_ref(|| 0);
    let size = use_mut_ref(|| pager::DEFAULT_PAGE_SIZE);
    let data = use_mut_ref(|| Default::default());
    {
        let message = message.clone();
        let key_word = key_word.clone();
        let index = index.clone();
        let size = size.clone();
        let total = total.clone();
        let loading = loading.clone();
        let data = data.clone();
        let force_update = force_update.clone();
        let refresh = refresh.clone();
        let selected_row = selected_row.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    match user_controller_api::search(
                        &common::get_cli_config().unwrap(),
                        key_word.borrow().as_str(),
                        *index.borrow(),
                        *size.borrow() as i64,
                    )
                    .await
                    {
                        Ok(res) => {
                            *data.borrow_mut() = res.data;
                            *total.borrow_mut() = res.total;
                            *loading.borrow_mut() = false;
                            force_update.force_update();
                        }
                        Err(err) => {
                            *message.borrow_mut() = Some(message_list::error(&format!("{}", err)));
                            *loading.borrow_mut() = false;
                        }
                    };
                });
                *selected_row.borrow_mut() = None;
            },
            refresh,
        )
    }

    let user_form_close = {
        let user_form_closed = user_form_closed.clone();
        let force_update = force_update.clone();
        Callback::from(move |_e| {
            *user_form_closed.borrow_mut() = true;
            force_update.force_update();
        })
    };

    let user_form_update = {
        let user_form_closed = user_form_closed.clone();
        let refresh = refresh.clone();
        let index = index.clone();
        Callback::from(move |_e| {
            *user_form_closed.borrow_mut() = true;
            *index.borrow_mut() = 1;
            refresh.set(!*refresh);
        })
    };

    let confirm_form_close = {
        let confirm_form_closed = confirm_form_closed.clone();
        let force_update = force_update.clone();
        Callback::from(move |_e| {
            *confirm_form_closed.borrow_mut() = true;
            force_update.force_update();
        })
    };

    let confirm_form_confirm = {
        let confirm_form_closed = confirm_form_closed.clone();
        let refresh = refresh.clone();
        let index = index.clone();
        Callback::from(move |_e| {
            *confirm_form_closed.borrow_mut() = true;
            *index.borrow_mut() = 1;
            refresh.set(!*refresh);
        })
    };

    let key_word_ref = use_node_ref();

    let key_word_change = {
        let key_word_ref = key_word_ref.clone();
        let key_word = key_word.clone();
        let refresh = refresh.clone();
        let index = index.clone();
        Callback::from(move |_| {
            let input = key_word_ref.cast::<web_sys::HtmlInputElement>();
            if let Some(input) = input {
                *key_word.borrow_mut() = input.value();
                *index.borrow_mut() = 1;
                refresh.set(!*refresh);
            }
        })
    };

    let edit = {
        let selected_row = selected_row.clone();
        let message = message.clone();
        let user_form_closed = user_form_closed.clone();
        let force_update = force_update.clone();
        Callback::from(move |_e| {
            if selected_row.borrow().is_none() {
                *message.borrow_mut() = Some(message_list::warn("please select a record"));
            } else {
                *user_form_closed.borrow_mut() = false;
            }
            force_update.force_update()
        })
    };

    let delete = {
        let selected_row = selected_row.clone();
        let message = message.clone();
        let confirm_form_closed = confirm_form_closed.clone();
        let force_update = force_update.clone();
        Callback::from(move |_e| {
            if selected_row.borrow().is_none() {
                *message.borrow_mut() = Some(message_list::warn("please select a record"));
            } else {
                *confirm_form_closed.borrow_mut() = false;
            }
            force_update.force_update()
        })
    };

    let page_change = {
        let index = index.clone();
        let size = size.clone();
        let refresh = refresh.clone();
        Callback::from(move |page: Page| {
            *index.borrow_mut() = page.index as i64;
            *size.borrow_mut() = page.size;
            refresh.set(!*refresh);
        })
    };

    let selected_id = selected_row.borrow().clone().map(|x| x.id);
    html! {
    <>
    <MessageList value = {(*message.borrow()).clone()} ws = true/>
    if let Some(v) = (*selected_row.clone().borrow()).clone()  {
        if !(*user_form_closed.borrow()){
            <UserForm value = {v.clone()} onclose={user_form_close} onupdate = {user_form_update}/>
        }
        if !(*confirm_form_closed.borrow()){
            <ConfirmForm onclose = {confirm_form_close} onconfirm = {confirm_form_confirm.clone()} content = {"Deleted users <b>can not</b> be recovered!!!<br/> are you sure you want to delete it?"}/>
        }
    }
    <div class="search-container">
        <div class="search-input field is-grouped">
        <p class="control is-expanded">
            <input ref={key_word_ref} class="input" type="text" onkeyup={key_word_change} placeholder="Search"/>
        </p>

        <p class="control">
            <button class="button is-light is-warning" onclick={edit}>{"Edit"}</button>
        </p>
        <p class="control">
            <button class="button is-light is-danger" onclick={delete}>{"Delete"}</button>
        </p>
        </div>
    </div>
    <div class="table-container">
        {
            if *loading.borrow() {
                html!{
                    <div class="table-loading">
                    </div>
                }
            }else{
                html!{}
            }
        }

        <table class="table is-bordered is-striped is-narrow is-hoverable">
        <thead>
            <tr>
            <th><abbr title="Type">{"Type"}</abbr></th>
            <th><abbr title="Email">{"Email"}</abbr></th>
            <th><abbr title="Name">{"Name"}</abbr></th>
            <th><abbr title="Mobile">{"Mobile"}</abbr></th>
            <th><abbr title="Laston">{"Laston"}</abbr></th>
            // todo: sort
            <th><abbr title="Created_at"><a href="javascript:void(0)">{"Created_at  "}<i class="fa-solid fa-arrow-down"></i></a></abbr></th>
            <th><abbr title="Updated_at">{"Updated_at"}</abbr></th>
            <th><abbr title="Status">{"Status"}</abbr></th>
            </tr>
        </thead>
        <tbody>
        {
            data.clone().borrow().iter().map(|x|{
                let user = *(x.user.clone());
                let formatter = *(x.formatter.clone());
                let select_row = {
                    let selected_row = selected_row.clone();
                    let force_update = force_update.clone();
                    let user = user.clone();
                    Callback::from(move |_| {
                        let u = &user;
                        *selected_row.borrow_mut() = Some(u.clone());
                        force_update.force_update();
                    })
                };
                let mut is_selected = false;
                if let Some(id) = selected_id{
                    if id == x.user.id {
                        is_selected = true;
                    }
                }
                html! {
                    <tr class = {if is_selected {"is-selected"} else {""}}
                     onclick = {select_row} >
                        {common::create_html("td",formatter.r#type.as_str())}
                        {common::create_html("td",formatter.email.as_str())}
                        {common::create_html("td",formatter.name.as_str())}
                        {common::create_html("td",formatter.mobile.as_str())}
                        {common::create_html("td",formatter.laston.as_str())}
                        {common::create_html("td",formatter.created_at.as_str())}
                        {common::create_html("td",formatter.updated_at.as_str())}
                        {common::create_html("td",formatter.status.as_str())}
                    </tr>
                }
        }).collect::<Html>()
        }
        </tbody>
        </table>
    </div>
    <div class="pager-container">
    {
        html!{
            <Pager total = { *total.borrow() as usize } index = {*index.borrow() as usize} onpagechanged = {page_change}/>
        }
    }
    </div>

    </>
                    }
}

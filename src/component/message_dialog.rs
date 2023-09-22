use crate::component::menu::{Menu, MenuLabel, MenuNode};
use crate::component::message_list::MessageContent;
use futures::stream::SplitSink;
use futures::SinkExt;
use gloo_net::websocket::{futures::WebSocket, Message};
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::Properties;

#[derive(Clone, Properties)]
pub struct MessageDialogProps {
    #[prop_or_default]
    pub rooms: Rc<RefCell<HashMap<String, HashMap<String, String>>>>,

    #[prop_or_default]
    pub session_id: String,

    #[prop_or_default]
    pub messages: Rc<RefCell<HashMap<String, LinkedList<MessageContent>>>>,

    #[prop_or_default]
    pub ws_writer: Rc<RefCell<Option<SplitSink<WebSocket, Message>>>>,

    #[prop_or_default]
    pub current_room: Rc<RefCell<Option<String>>>,

    pub onclose: Callback<()>,
}

impl PartialEq for MessageDialogProps {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

#[function_component(MessageDialog)]
pub fn message_dialog(props: &MessageDialogProps) -> Html {
    let force_update = use_force_update();
    let borrow = props.current_room.borrow();
    let title = borrow.as_deref().unwrap_or("Dialog");
    let closedialog = {
        let onclose = props.onclose.clone();
        Callback::from(move |_| {
            onclose.emit(());
        })
    };

    let room_nodes = props
        .rooms
        .borrow()
        .iter()
        .map(|(room, _)| MenuNode {
            name: room.to_string(),
            children: vec![],
        })
        .collect::<Vec<MenuNode>>();

    let room_labels = vec![MenuLabel {
        label: None,
        nodes: room_nodes,
    }];

    let click_room = {
        let current_room = props.current_room.clone();
        let force_update = force_update.clone();
        Callback::from(move |name| {
            *current_room.borrow_mut() = Some(name);
            force_update.force_update();
        })
    };

    let ref1 = use_node_ref();
    let ref2 = use_node_ref();

    let key_send = {
        let force_update = force_update.clone();
        let ref2 = ref2.clone();
        let messages = props.messages.clone();
        let current_room = props.current_room.clone();
        let ws_writer = props.ws_writer.clone();
        let session_id = props.session_id.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key_code() == 13 {
                let message_input = &ref2;
                let input = message_input.cast::<HtmlInputElement>().unwrap();
                let content = input.value();
                if !e.meta_key() {
                    // dot not with ALT
                    e.prevent_default();
                    if !content.trim_matches('\n').trim().is_empty() {
                        if let Some(room) = current_room.borrow().as_deref() {
                            let room = room.to_string();
                            let session_id = session_id.clone();
                            let messages = messages.clone();
                            let force_update = force_update.clone();
                            let ws_writer = ws_writer.clone();
                            spawn_local(async move {
                                if let Some(ss) = &mut *ws_writer.borrow_mut() {
                                    ss.send(Message::Text(content.clone())).await.unwrap();
                                    messages
                                        .borrow_mut()
                                        .entry(room.clone())
                                        .or_insert(Default::default())
                                        .push_back(MessageContent {
                                            id: 0,
                                            room: room.to_string(),
                                            from_id: "".to_string(),
                                            from_name: session_id,
                                            content: content.clone(),
                                            time: "".to_string(), //chrono::Utc::now().to_default(),
                                            is_own: Some(()),
                                        });
                                    force_update.force_update()
                                }
                            });
                        }
                    }
                    input.set_value("");
                } else {
                    input.set_value(&(input.value() + "\n"));
                }
            }
            // false
        })
    };

    let mut session_nodes = vec![];

    if let Some(room) = props.current_room.borrow().as_deref() {
        if let Some(sessions) = props.rooms.borrow().get(room) {
            for (_, name) in sessions.iter() {
                session_nodes.push(MenuNode {
                    name: name.to_string(),
                    children: vec![],
                });
            }
        }
    }

    let session_labels = vec![MenuLabel {
        label: None,
        nodes: session_nodes,
    }];

    {
        let messages = props.messages.clone();
        let message_input = ref1.clone();
        let current_room = props.current_room.clone();
        use_effect(move || {
            message_input
                .cast::<HtmlInputElement>()
                .and_then::<(), _>(|input| {
                    let msg = match current_room.borrow().as_deref() {
                        Some(room) => messages
                            .borrow()
                            .get(room)
                            .unwrap_or(&Default::default())
                            .iter()
                            .map(|x| format!("{}: {}\n\n", x.from_name, x.content))
                            .collect::<String>(),
                        None => "".to_string(),
                    };
                    input.set_value(&msg);
                    input.set_scroll_top(input.scroll_height());
                    None
                });
        });
    }
    let current_room = (&*props.current_room.borrow()).clone();

    html! {
        <div class="modal is-active">
            <div class="modal-background"></div>
            <div class="modal-card" style="height:70%;width:60%;">
                <header class="modal-card-head">
                <p class="modal-card-title">{title}</p>
                <button class="delete" aria-label="close" onclick={closedialog}></button>
                </header>

                <section class="modal-card-body">
                <div class="columns" style="height:100%;">
                <div class="column is-2">
                    <div style="height: 100%; overflow: scroll;">
                        <Menu labels = {room_labels} selected_name = {current_room} onselect = {click_room}/>
                    </div>
                </div>
                <div class="column is-7">
                    <div style="height: 70%;">
                        <textarea ref={ref1} style="height: 100%;" readonly={true} class="textarea has-fixed-size"></textarea>
                    </div>
                    <div style="margin-top: 0.8em;">
                        <textarea ref={ref2} class="textarea has-fixed-size" onkeydown={key_send} />
                    </div>
                </div>
                <div class="column is-3">
                    <div style="height: 100%; overflow: scroll;">
                        <Menu labels = {session_labels}/>
                    </div>
                </div>
                </div>
                </section>
            </div>
        </div>
    }
}

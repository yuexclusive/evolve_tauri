#![allow(dead_code)]

use super::message_item::{MessageItem, MessageItemType, MessageItemValue};
use serde::Deserialize;
use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;
use yew::Properties;

use super::message_dialog::MessageDialog;
use crate::util::request;
use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message};
use std::collections::HashMap;
use std::collections::LinkedList;
use wasm_bindgen_futures::spawn_local;

const DEFAULT_ROOM: &str = "main";
const UPDATE_SESSION_PRE: &str = "update_session:";
const LIST_PRE: &str = "list:";
const JOIN_ROOM_PRE: &str = "join_room:";
const QUIT_ROOM_PRE: &str = "quit_room:";
const UPDATE_NAME_PRE: &str = "update_name:";
const MESSAGE_PRE: &str = "message:";

#[derive(Deserialize)]
struct UpdateSession<'a> {
    pub room: &'a str,
    pub name: &'a str,
}

#[derive(Deserialize)]
struct UpdateName<'a> {
    pub session_id: &'a str,
    pub name: &'a str,
    pub old_name: &'a str,
}

#[derive(Deserialize, Debug)]
struct RoomChange<'a> {
    pub session_id: &'a str,
    pub name: &'a str,
    pub room: &'a str,
}

#[derive(Clone, Properties)]
pub struct MessageListProps {
    #[prop_or_default]
    pub value: Option<MessageItemValue>,
    #[prop_or_default]
    pub ws: bool,
}

pub type MessageListValue = LinkedList<MessageItemValue>;

impl PartialEq for MessageListProps {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct MessageContent {
    pub id: u128,
    pub room: String,
    pub from_id: String,
    pub from_name: String,
    pub content: String,
    pub time: String,
    pub is_own: Option<()>,
}

pub fn ok(msg: &str) -> MessageItemValue {
    MessageItemValue::new(
        MessageItemType::Success,
        "Success",
        msg,
        Some(5),
        None,
        None,
    )
}

pub fn warn(msg: &str) -> MessageItemValue {
    MessageItemValue::new(
        MessageItemType::Warning,
        "Warning",
        msg,
        Some(10),
        None,
        None,
    )
}

pub fn info(msg: &str) -> MessageItemValue {
    MessageItemValue::new(MessageItemType::Info, "Info", msg, Some(8), None, None)
}

pub fn error(msg: &str) -> MessageItemValue {
    MessageItemValue::new(MessageItemType::Danger, "Error", msg, None, None, None)
}

pub fn message(room: &str, from_id: &str, from_name: &str, content: &str) -> MessageItemValue {
    MessageItemValue::new(
        MessageItemType::Primary,
        room,
        content,
        None,
        Some(from_name),
        Some(from_id),
    )
}

#[function_component(MessageList)]
pub fn message_list(props: &MessageListProps) -> Html {
    let force_update = use_force_update();
    let dialog_closed: Rc<RefCell<bool>> = use_mut_ref(|| true);
    let ws_writer: Rc<RefCell<Option<SplitSink<WebSocket, Message>>>> =
        use_mut_ref(|| Default::default());
    let rooms: Rc<RefCell<HashMap<String, HashMap<String, String>>>> =
        use_mut_ref(|| Default::default());
    let session_id: Rc<RefCell<Option<String>>> = use_mut_ref(|| Default::default());
    let text_messages: Rc<RefCell<HashMap<String, LinkedList<MessageContent>>>> =
        use_mut_ref(|| Default::default());
    let current_room: Rc<RefCell<Option<String>>> = use_mut_ref(|| Default::default());
    let message_list = use_mut_ref(|| MessageListValue::new());

    {
        let message_list = message_list.clone();
        let value = props.value.clone();
        use_memo(
            move |value| {
                if let Some(v) = value {
                    message_list.borrow_mut().push_back(v.clone());
                }
            },
            value,
        );
    }

    {
        let force_update = force_update.clone();
        let dialog_closed = dialog_closed.clone();
        let ws = props.ws;
        let ws_writer = ws_writer.clone();
        let text_messages = text_messages.clone();
        let self_rooms = rooms.clone();
        let session_id = session_id.clone();
        let session_id_c = session_id.clone();
        let message_list = message_list.clone();
        let current_room = current_room.clone();
        // depends on (), only effected once
        use_effect_with_deps(
            move |_| {
                if ws {
                    let ws = request::open_ws().unwrap();
                    let (writer, mut reader) = ws.split();
                    *ws_writer.borrow_mut() = Some(writer);
                    let w1 = Rc::clone(&ws_writer);
                    spawn_local(async move {
                        if let Some(ss) = &mut *w1.borrow_mut() {
                            ss.send(Message::Text(String::from("i am back online!")))
                                .await
                                .unwrap();
                        }
                    });

                    let mut sid = session_id_c.borrow_mut();
                    if sid.is_none() {
                        let user = &crate::util::common::get_current_user().unwrap();
                        *sid = Some(user.name.clone().unwrap_or(user.email.clone()));
                    }
                    spawn_local(async move {
                        while let Some(msg) = reader.next().await {
                            match msg {
                                Ok(msg) => {
                                    // log::info!("{:?}", msg);
                                    // 【{room}】{name}: {msg}
                                    if let Message::Text(content) = msg {
                                        if content.starts_with(MESSAGE_PRE) {
                                            let message_content: MessageContent =
                                                serde_json::from_str(
                                                    content.trim_start_matches(MESSAGE_PRE),
                                                )
                                                .unwrap();
                                            text_messages
                                                .borrow_mut()
                                                .entry(message_content.room.clone())
                                                .or_insert(Default::default())
                                                .push_back(message_content.clone());

                                            if *dialog_closed.borrow() {
                                                message_list.borrow_mut().push_back(message(
                                                    &message_content.room,
                                                    &message_content.from_id,
                                                    &message_content.from_name,
                                                    &message_content.content,
                                                ));
                                            }
                                        } else {
                                            if content.starts_with(UPDATE_SESSION_PRE) {
                                                let change: UpdateSession = serde_json::from_str(
                                                    content.trim_start_matches(UPDATE_SESSION_PRE),
                                                )
                                                .unwrap();

                                                *current_room.borrow_mut() =
                                                    Some(change.room.to_string());
                                            } else if content.starts_with(LIST_PRE) {
                                                let rooms: HashMap<
                                                    String,
                                                    HashMap<String, String>,
                                                > = serde_json::from_str(
                                                    content.trim_start_matches(LIST_PRE),
                                                )
                                                .unwrap();

                                                *self_rooms.borrow_mut() = rooms;
                                            } else if content.starts_with(JOIN_ROOM_PRE) {
                                                let change: RoomChange = serde_json::from_str(
                                                    content.trim_start_matches(JOIN_ROOM_PRE),
                                                )
                                                .unwrap();
                                                let mut sr = self_rooms.borrow_mut();

                                                sr.entry(change.room.to_string())
                                                    .or_default()
                                                    .insert(
                                                        change.session_id.to_string(),
                                                        change.name.to_string(),
                                                    );
                                            } else if content.starts_with(QUIT_ROOM_PRE) {
                                                let change: RoomChange = serde_json::from_str(
                                                    content.trim_start_matches(QUIT_ROOM_PRE),
                                                )
                                                .unwrap();

                                                let mut sr = self_rooms.borrow_mut();

                                                if let Some(current_session_id) =
                                                    &*session_id.borrow()
                                                {
                                                    if current_session_id == change.session_id {
                                                        sr.remove(change.room);
                                                    } else {
                                                        sr.get_mut(change.room).and_then(|x| {
                                                            x.remove(change.session_id)
                                                        });
                                                    }
                                                }
                                            } else if content.starts_with(UPDATE_NAME_PRE) {
                                                let change: UpdateName = serde_json::from_str(
                                                    content.trim_start_matches(UPDATE_NAME_PRE),
                                                )
                                                .unwrap();

                                                for (_, sessions) in &mut *self_rooms.borrow_mut() {
                                                    sessions
                                                        .entry(change.session_id.to_string())
                                                        .and_modify(|x| {
                                                            *x = change.name.to_string()
                                                        });
                                                }
                                            }
                                        }
                                        force_update.force_update();
                                    }
                                }
                                Err(err) => match err {
                                    gloo_net::websocket::WebSocketError::ConnectionError => {
                                        log::error!("connection error: {:#?}", err);
                                        break;
                                    }
                                    gloo_net::websocket::WebSocketError::ConnectionClose(e) => {
                                        log::info!("connection closed, close event: {:#?}", e);
                                        break;
                                    }
                                    gloo_net::websocket::WebSocketError::MessageSendError(e) => {
                                        log::error!("message send error: {:#?}", e);
                                    }
                                    _ => {
                                        log::error!("read error: {:#?}", err);
                                    }
                                },
                            }
                        }
                    });
                }
            },
            (),
        );
    }

    let on_close = {
        let dialog_closed = dialog_closed.clone();
        let force_update = force_update.clone();
        Callback::from(move |_| {
            *dialog_closed.borrow_mut() = true;
            force_update.force_update()
        })
    };
    let list = &*message_list.borrow();

    html! {
        <>
        <div class="message-list">
        {
            list.iter().map(|x|{
                let open_dialog = {
                    let dialog_closed = dialog_closed.clone();
                    let value = message_list.clone();
                    let current_room = current_room.clone();
                    let force_update = force_update.clone();
                    let item = x.clone();
                    Callback::from(move |_| {
                        let item = item.clone();
                        *dialog_closed.borrow_mut() = false;
                        *current_room.borrow_mut() = Some(item.room);
                        value.borrow_mut().clear();
                        force_update.force_update();
                    })
                };

                let close_message_item = {
                    let value = message_list.clone();
                    let force_update = force_update.clone();
                    Callback::from(move |id| {
                       let index = value
                            .borrow()
                            .iter()
                            .enumerate()
                            .find(|&(_, v)| v.id == id)
                            .map(|(index, _)| index);
                        if let Some(index) = index{
                            value.borrow_mut().remove(index);
                            force_update.force_update();
                        }
                    })
                };
                html!{
                <MessageItem value = {x.clone()} onclose={close_message_item} onopendialog={open_dialog}/>
            }}).collect::<Html>()
        }
        </div>
        {
            if !*dialog_closed.borrow() {
                let session_id = session_id.borrow().clone().unwrap();
                html!{
                    <MessageDialog session_id={session_id} rooms={rooms.clone()} messages={text_messages.clone()}  ws_writer = {ws_writer.clone()} onclose={on_close} current_room = { current_room }/>
                }
            }else{
                html!{}
            }
        }
        </>
    }
}

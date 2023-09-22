use gloo::timers::callback::Timeout;
use uuid::Uuid;
use yew::prelude::*;
use yew::Properties;

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone)]
pub enum MessageItemType {
    Dark,
    Primary,
    Link,
    Info,
    Success,
    Warning,
    Danger,
}

#[derive(PartialEq, Debug, Clone)]
pub struct MessageItemValue {
    pub id: u128,
    pub from_id: Option<String>,
    pub from: Option<String>,
    pub content: String,
    pub room: String,
    pub r#type: MessageItemType,
    // seconds
    pub timeout: Option<u32>,
}

#[derive(PartialEq, Properties, Debug)]
pub struct MessageItemProps {
    pub value: MessageItemValue,
    pub onclose: Callback<u128>,
    pub onopendialog: Callback<u128>,
}

impl MessageItemValue {
    pub fn new(
        r#type: MessageItemType,
        room: &str,
        content: &str,
        timeout: Option<u32>,
        from: Option<&str>,
        from_id: Option<&str>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().as_u128(),
            r#type,
            room: room.to_string(),
            content: content.to_string(),
            from_id: from_id.and_then(|x| Some(x.to_string())),
            from: from.and_then(|x| Some(x.to_string())),
            timeout,
        }
    }
}

#[function_component(MessageItem)]
pub fn message_item(props: &MessageItemProps) -> Html {
    let value = props.value.clone();
    let content = value.content.trim_matches('"');
    let content = match &value.from {
        Some(name) => format!("{name}: {content}"),
        None => content.to_string(),
    };
    let id = value.id;

    let class_t = format!("{:?}", &value.r#type).to_lowercase();
    let cursor_style = if value.r#type == MessageItemType::Primary {
        "cursor: pointer"
    } else {
        ""
    };

    let close = {
        let onclose = props.onclose.clone();
        Callback::from(move |_| onclose.emit(id))
    };

    let open_dialog = {
        if value.r#type == MessageItemType::Primary {
            let on_open_dialog = props.onopendialog.clone();
            Callback::from(move |_| on_open_dialog.emit(id))
        } else {
            Callback::from(|_| {})
        }
    };

    if let Some(timeout) = value.timeout {
        let onclose = props.onclose.clone();
        Timeout::new(1000 * timeout, move || onclose.emit(id)).forget();
    }

    html! {
        <article class={format!{"message is-light is-small is-{}", class_t}}>
        <div class="message-header">
            <p>{value.room.clone()}</p>
            <button class="delete" aria-label="delete" onclick = {close}></button>
        </div>
        <div class="message-body"  style={cursor_style} onclick = {open_dialog}>
           {content}
        </div>
        </article>
    }
}

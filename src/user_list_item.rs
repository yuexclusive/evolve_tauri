// use crate::util::common;
// use lazy_static::__Deref;
// use user_cli::models::{SearchedUser, User};
// use yew::prelude::*;
// use yew::Properties;

// // pub struct UserListItem;

// #[derive(Clone, PartialEq, Properties)]
// pub struct UserListItemProps {
//     pub value: SearchedUser,
//     #[prop_or_default]
//     pub is_selected: bool,
//     pub onselect: Callback<()>,
// }

// #[function_component(UserListItem)]
// pub fn user_list_item(props: &UserListItemProps) -> Html {
//     let formatter = &props.value.formatter;
//     let is_selected = props.is_selected;
//     // let user = props.value.user.deref().clone();
//     let a = String::from("hello");
//     let onclick = {
//         let onselect = props.onselect.clone();
//         let user1 = a.clone();
//         Callback::from(move |_| {
//             let a = user1;
//             // let user = user1;
//             onselect.emit(())
//         })
//     };
//     html! {
//         <tr class = {if is_selected {"is-selected"} else {""}}
//          onclick = {onclick} >
//             {common::create_html("td",formatter.r#type.as_str())}
//             {common::create_html("td",formatter.email.as_str())}
//             {common::create_html("td",formatter.name.as_str())}
//             {common::create_html("td",formatter.mobile.as_str())}
//             {common::create_html("td",formatter.laston.as_str())}
//             {common::create_html("td",formatter.created_at.as_str())}
//             {common::create_html("td",formatter.updated_at.as_str())}
//             {common::create_html("td",formatter.status.as_str())}
//         </tr>
//     }
// }

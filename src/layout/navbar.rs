use crate::util::common;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct NavbarProps {
    #[prop_or_default]
    pub selected_navbar_name: Option<String>,
    #[prop_or_default]
    pub selected_navbar_parent_name: Option<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct NavbarNode {
    name: String,
    children: Vec<NavbarNode>,
    path: Option<String>,
    divider: bool,
}

impl NavbarNode {
    fn render(&self, props: &NavbarProps) -> Html {
        let selected_navbar_name = props.selected_navbar_name.clone();
        let selected_navbar_parent_name = props.selected_navbar_parent_name.clone();
        if self.children.is_empty() {
            let onselect = {
                let path = self.path.clone().unwrap();
                Callback::from(move |_| {
                    common::redirect(&path);
                })
            };
            html! {
                <a href={String::from("javascript:void(0)")} onclick = {onselect} class={if selected_navbar_name.is_some() && &self.name == &selected_navbar_name.clone().unwrap()  {"navbar-item is-active"} else {"navbar-item"}}>
                    {self.name.clone()}
                </a>
            }
        } else {
            html! {
            <div class="navbar-item has-dropdown is-hoverable">
                    <a href={String::from("javascript:void(0)")} class={if selected_navbar_parent_name.is_some() && &self.name == &selected_navbar_parent_name.clone().unwrap() {"navbar-link is-active"} else {"navbar-link"}}>
                        {self.name.clone()}
                    </a>
                    <div class="navbar-dropdown">
                        {
                            self.children.iter().map(|child_item|{
                                let class = {if selected_navbar_name.is_some() && &child_item.name == &selected_navbar_name.clone().unwrap() {"navbar-item is-active"} else {"navbar-item"}};
                                let onselect = {
                                    let path = child_item.path.clone().unwrap();
                                    Callback::from(move |_| {
                                        common::redirect(&path);
                                    })
                                };
                                html!{
                                    <>
                                    <a href={String::from("javascript:void(0)")} onclick = {onselect} class={class} >
                                        {child_item.name.clone()}
                                    </a>
                                    {
                                        if child_item.divider{
                                        html!{ <hr class="navbar-divider"/> }
                                        }else{
                                        html!{}
                                        }
                                    }
                                    </>
                                }
                            }).collect::<Html>()
                        }
                    </div>
            </div>
            }
        }
    }
}

#[function_component(Navbar)]
pub fn navbar(props: &NavbarProps) -> Html {
    let data = vec![NavbarNode {
        name: "Modules".to_string(),
        path: None,
        divider: false,
        children: vec![
            NavbarNode {
                name: "Welcome".to_string(),
                path: Some("/".to_string()),
                divider: true,
                children: vec![],
            },
            NavbarNode {
                name: "User".to_string(),
                path: Some("/main/user".to_string()),
                divider: false,
                children: vec![],
            },
        ],
    }];
    html! {
        <div class="navbar-start">
        {
            data
            .iter()
            .map(|item| {
                item.render(props)
            })
            .collect::<Html>()
        }
        </div>
    }
}
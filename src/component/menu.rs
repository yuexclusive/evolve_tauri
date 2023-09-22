use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew::Properties;

// pub struct Menu;

#[derive(Clone, Properties, PartialEq)]
pub struct MenuProps {
    #[prop_or_default]
    pub labels: Vec<MenuLabel>,
    #[prop_or_default]
    pub onselect: Callback<String>,
    #[prop_or_default]
    pub selected_name: Option<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct MenuLabel {
    pub label: Option<String>,
    pub nodes: Vec<MenuNode>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct MenuNode {
    pub name: String,
    pub children: Vec<MenuNode>,
}

impl MenuNode {
    fn render(&self, props: &MenuProps) -> Html {
        let mut class = "";
        if let Some(v) = props.selected_name.as_deref() {
            if v == &self.name {
                class = "is-active"
            }
        }

        let onclick = {
            let onselect = props.onselect.clone();
            let name = self.name.clone();
            Callback::from(move |_| onselect.emit(name.clone()))
        };

        html! {
            <li>
                <a href={String::from("javascript:void(0)")} class={class} onclick = {onclick}>{&self.name}</a>
                {
                    if self.children.is_empty() {
                        html!{}
                    } else{
                        html!{
                            <ul>
                                {
                                    self.children.iter().map(|n|n.render(props)).collect::<Html>()
                                }
                            </ul>
                        }
                    }
                }
            </li>
        }
    }
}

#[function_component(Menu)]
pub fn menu(props: &MenuProps) -> Html {
    html! {
        <aside class="menu">
    {
        props.labels.iter().map(|x|html!{
        <>
            {
                if let Some(label) = &x.label{
                    html!{
                        <p class="menu-label">
                        {label}
                        </p>
                    }
                } else{
                    html!{}
                }
            }
            <ul class="menu-list">
                {
                    x.nodes.iter().map(|n|html!{
                        n.render(props)
                    }).collect::<Html>()
                }
            </ul>
        </>}).collect::<Html>()
    }
    </aside>
    }
}

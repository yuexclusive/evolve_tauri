use crate::layout::navbar::Navbar;
use crate::util::common;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    #[prop_or_default]
    pub selected_navbar_name: Option<String>,
    #[prop_or_default]
    pub selected_navbar_parent_name: Option<String>,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let mut user = None;
    match common::get_current_user() {
        Ok(v) => user = Some(v),
        Err(err) => {
            log::warn!("get current user error: {}", err);
            // common::redirect("/401");
            common::redirect("/login");
        }
    };
    let user = user.unwrap();
    let navbar_active = use_state(|| false);
    let navbar_active_class = if *navbar_active { "is-active" } else { "" };
    let toggle_navbar_active = {
        let navbar_active = navbar_active.clone();
        Callback::from(move |_| {
            navbar_active.set(!*navbar_active);
        })
    };
    let logout = {
        Callback::from(move |_| {
            common::delete_current_user().unwrap_or_else(|x| {
                log::error!("{:?}", x);
            });
            common::redirect("/login");
        })
    };
    html! {
        <div class="header-container">
            <nav class="navbar is-light" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <a class="navbar-item" href="/">
                        <img alt="fuck you" src="/static/img/logo.png" width="100" height="100"/>
                    </a>

                    <a href={String::from("javascript:void(0)")} role="button" onclick={toggle_navbar_active} class={format!{"navbar-burger {navbar_active_class}"}} aria-label="menu" aria-expanded="false" data-target="navbarBasicExample">
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    </a>
                </div>

                <div id="navbarBasicExample" class={format!("navbar-menu {navbar_active_class}")}>
                    <Navbar selected_navbar_name={props.selected_navbar_name.clone()} selected_navbar_parent_name={props.selected_navbar_parent_name.clone()}/>
                    <div class="navbar-end">
                        <div class="navbar-item has-dropdown is-hoverable">
                            <a href={String::from("javascript:void(0)")} class="navbar-link" style="color:#000000">
                                { user.name.unwrap_or("unnamed".to_string())}
                            </a>

                            <div class="navbar-dropdown is-right">
                            <a href={String::from("javascript:void(0)")} class="navbar-item">
                                {user.r#type}
                            </a>
                            <a href={String::from("javascript:void(0)")} class="navbar-item">
                                {user.email}
                            </a>
                            <hr class="navbar-divider"/>
                            <a href={String::from("javascript:void(0)")} onclick={logout} class="navbar-item">
                                {"Logout"}
                            </a>
                            </div>
                        </div>
                    </div>
                </div>
            </nav>
        </div>
    }
}

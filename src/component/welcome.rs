use gloo::timers::callback::Timeout;
use yew::prelude::*;
use yew::Properties;

#[derive(Properties, PartialEq)]
pub struct WelcomeProps<'a> {
    pub greeting: &'a str,
}

#[function_component(Welcome)]
pub fn welcome(props: &WelcomeProps<'static>) -> Html {
    let now = use_state(|| chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());

    {
        let now = now.clone();
        Timeout::new(1000, move || {
            now.set(chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string())
        })
        .forget();
    }

    html! {
        <div class="search-container">
            <p><b>{props.greeting}</b></p>
            <hr/>
            <p>{&*now}</p>
        </div>
    }
}

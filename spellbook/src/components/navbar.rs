use yew::prelude::*;
use yew_router::prelude::*;

use crate::MainRoute;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let navigator = use_navigator().unwrap();

    let go_home_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&MainRoute::Home));
        html! {
            <button {onclick}>{"Home"}</button>
        }
    };

    let go_to_search_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&MainRoute::Search));
        html! {
            <button {onclick}>{"Search"}</button>
        }
    };

    let go_to_modify_button = {
        let onclick = Callback::from(move |_| navigator.push(&MainRoute::Modify));
        html! {
            <button {onclick}>{"Modify"}</button>
        }
    };

    html! {
        <div>
            {go_home_button}
            {go_to_search_button}
            {go_to_modify_button}
        </div>
    }
}

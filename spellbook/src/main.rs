use components::navbar::Navbar;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;

#[derive(Clone, Routable, PartialEq)]
enum MainRoute {
    #[at("/")]
    Home,
    #[at("/search")]
    Search,
    #[at("/modify")]
    Modify,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch_main(route: MainRoute) -> Html {
    match route {
        MainRoute::Home => html! {<h1>{"Spellbook"}</h1>},
        MainRoute::Search => html! {<h1>{"Search"}</h1>},
        MainRoute::Modify => html! {<h1>{"Modify"}</h1>},
        MainRoute::NotFound => html! {<h1>{"Not Found"}</h1>},
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Navbar/>
            <Switch<MainRoute> render={switch_main} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

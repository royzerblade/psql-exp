use crate::components::navbar::Navbar;
use crate::pages::home::Home;
use crate::pages::modify::Modify;
use crate::pages::search::Search;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;

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
        MainRoute::Home => html! { <Home/> },
        MainRoute::Search => html! { <Search/> },
        MainRoute::Modify => html! { <Modify/> },
        MainRoute::NotFound => html! {<h1>{"404: Page Not Found"}</h1>},
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

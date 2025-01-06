use crate::components::spellform::Spellform;
use yew::prelude::*;

#[function_component(Modify)]
pub fn modify() -> Html {
    html! {
      <div>
        <h1>{"Modify Component"}</h1>
        <Spellform />
        <button>{"Add to DB"}</button>
      </div>
    }
}

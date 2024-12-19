use crate::api::get::get;
use shared::types::spell::QuerySpell;
use yew::{platform::spawn_local, prelude::*};

#[function_component(Search)]
pub fn search() -> Html {
    let spell_list: UseStateHandle<Vec<QuerySpell>> = use_state(|| vec![]);

    let onclick = {
        Callback::from(move |_| {
            spawn_local(async move {
                spell_list.set(get().await);
            });
        })
    };

    let search_button = {
        html! {
          <button {onclick}>{"Go!"}</button>
        }
    };

    html! {
      <div>
        <h1>{"Search Component"}</h1>
        {search_button}
        <ul>
            { for spell_list.iter().map(|spell| html! {
                <li>{ spell.description.clone() }</li>
            }) }
        </ul>
      </div>
    }
}

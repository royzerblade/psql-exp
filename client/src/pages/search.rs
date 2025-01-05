use crate::api::get::get;
use crate::components::spellcard::Spellcard;
use shared::types::spell::QuerySpell;
use yew::prelude::*;

#[function_component(Search)]
pub fn search() -> Html {
    let spell_list: UseStateHandle<Vec<QuerySpell>> = use_state(|| vec![]);

    let onclick = {
        let spell_list = spell_list.clone();
        Callback::from(move |_| {
            let spell_list = spell_list.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let result = get().await;
                spell_list.set(result);
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
                <Spellcard spell={spell.clone()} />
            }) }
        </ul>
      </div>
    }
}

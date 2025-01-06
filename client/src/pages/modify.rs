use crate::api::post::post;
use crate::components::spellform::Spellform;
use shared::types::spell::QuerySpell;
use yew::prelude::*;

#[function_component(Modify)]
pub fn modify() -> Html {
    let input_spell: UseStateHandle<QuerySpell> = use_state(|| QuerySpell::default());

    let onclick = {
        let input_spell = input_spell.clone();
        Callback::from(move |_| {
            let input_spell = input_spell.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let query_spell = (*input_spell).clone();
                let _result = post(query_spell).await;
                input_spell.set(QuerySpell::default());
            });
        })
    };

    html! {
      <div>
        <h1>{"Modify Component"}</h1>
        <Spellform />
        <button {onclick}>{"Add to DB"}</button>
      </div>
    }
}

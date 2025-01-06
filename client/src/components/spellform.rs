use shared::types::spell::QuerySpell;
use std::collections::BTreeMap;
use structmap::{value::Value, ToMap};
use yew::{function_component, html, Html};

#[function_component(Spellform)]
pub fn spellform() -> Html {
    let generic_query: QuerySpell = QuerySpell::default();
    let query_map: BTreeMap<String, Value> = QuerySpell::to_genericmap(generic_query);

    html! {
      <div>
        <h2>{"Spell Fields"}</h2>
        <form>
          { for query_map.iter().map(|(key, value)| html! {
            <div>
              <label for={key.clone()}>{format!("{}:", key.clone())}</label>
              <input type="text" id={key.clone()} name={key.clone()}/><br/>
            </div>
          })}
        </form>
      </div>
    }
}

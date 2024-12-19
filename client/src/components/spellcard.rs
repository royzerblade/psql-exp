use shared::types::spell::QuerySpell;
use yew::prelude::*;

#[derive(Clone, Eq, PartialEq, Properties)]
pub struct PassedProps {
    pub spell: QuerySpell,
}

#[function_component(Spellcard)]
pub fn spellcard(props: &PassedProps) -> Html {
    html! {
      <div>{props.spell.description.clone()}</div>
    }
}

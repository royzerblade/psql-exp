use shared::types::spell::Spell;
use yew::prelude::*;

#[derive(Clone, Eq, PartialEq, Properties)]
pub struct PassedProps {
    pub spell: Spell,
}

#[function_component(Spellcard)]
pub fn spellcard(props: &PassedProps) -> Html {
    html! {
      <div>{props.spell.description.clone()}</div>
    }
}

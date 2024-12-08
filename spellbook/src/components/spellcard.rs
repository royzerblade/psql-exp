use yew::prelude::*;

mod spell;
use types::spell::Spell;

#[derive(Clone, Eq, PartialEq, Properties)]
pub struct PassedProps {
    pub spell: Spell,
}

#[function_component(Spellcard)]
pub fn spellcard(props: &PassedProps) -> Html {
    html! {
      <div>{props.spell}</div>
    }
}

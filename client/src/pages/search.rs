use yew::prelude::*;

#[function_component(Search)]
pub fn search() -> Html {
    let p: UseStateHandle<Vec<QuerySpell>> = use_state(|| Vec::new());

    let search_button = {
        let async onclick = {
            let p = p.clone();

            Callback::from(move |_| p.set(backend::calls::read::read()))
        };

        html! {
          <button {onclick}>{"Go!"}</button>
        }
    };

    html! {
      <div>
        <h1>{"Search Component"}</h1>
        {search_button}
        <ul>
            { for p.iter().map(|spell| html! {
                <li>{ spell.description.clone() }</li>
            }) }
        </ul>
      </div>
    }
}

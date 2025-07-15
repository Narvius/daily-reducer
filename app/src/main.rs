use std::cell::RefCell;

use daily_reducer::{DailyReducer, GAMES};
use web_sys::{HtmlTextAreaElement, wasm_bindgen::JsCast as _};
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let warning = use_state(|| None);
    let reducer = use_state(|| RefCell::new(DailyReducer::new()));
    let result_ref = use_node_ref();

    let oninput = {
        let warning = warning.clone();
        let result_ref = result_ref.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok())
            {
                let mut reducer = reducer.borrow_mut();
                if reducer.insert(target.value().as_str()) {
                    result_ref
                        .cast::<HtmlTextAreaElement>()
                        .unwrap()
                        .set_value(reducer.to_forum_block().as_str());
                } else {
                    warning.set(Some(format!(
                        "Failed to detect game for:\n\n{}",
                        target.value()
                    )));
                }

                target.set_value("");
            }
        })
    };

    let onclick = {
        let warning = warning.clone();
        Callback::from(move |_| {
            warning.set(None);
        })
    };

    let games = GAMES
        .iter()
        .map(|game| {
            html! {
                <div key={game.name}><a target="_blank" href={game.url}>{ format!("# {}", game.name) }</a></div>
            }
        })
        .collect::<Html>();

    html! {
        <div id="root">
            <textarea {oninput} id="paste" rows=1 placeholder="Paste here"/>
            if let Some(warning) = &*warning {
                <div class="warning" style="white-space: pre-wrap;">
                    <p>{ warning }</p>
                    <button {onclick}>{ "✔️" }</button>
                </div>
            }
            { games }
            <textarea ref={result_ref} id="result" cols=80 rows=24 readonly=true/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
